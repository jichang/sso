use super::super::models::ResourceCollection;
use chrono::{Duration, Utc};
use oath::{totp_raw_now, HashType};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;
use serde_repr::*;
use uuid::Uuid;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::guards::client_addr::ClientAddr;
use super::super::guards::permission::Permissions;
use super::super::models::audit;
use super::super::models::audit::{
    ChangePassword, ChangePasswordActivity, ChangePasswordActivityDetails, Signin, SigninActivity,
    SigninActivityDetails,
};
use super::super::models::group::GroupId;
use super::super::models::preference;
use super::super::models::preference::{Preference, PreferenceKey};
use super::super::models::resource::{ActionType, ResourceType};
use super::super::models::totp;
use super::super::models::user;
use super::super::models::user::User;
use super::super::models::PaginatorParams;
use super::super::storage::Database;
use super::Error;

pub const UNION_ID_LEN: usize = 32;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum AuthState {
    PASS = 0,
    NEED_TOTP = 1,
    NEED_WEBAUTHN = 2,
}

#[derive(Serialize, Deserialize)]
pub struct Auth {
    state: AuthState,
    user: Option<User>,
    jwt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SignupParams {
    username: String,
    password: String,
    invitation_code: String,
}

#[post("/signup", data = "<params>")]
pub fn signup(
    mut cookies: Cookies,
    config: State<Config>,
    db: State<Database>,
    client_addr: ClientAddr,
    params: Json<SignupParams>,
) -> Result<Created<Json<Auth>>, Error> {
    let conn = db.get_conn()?;
    let union_id = Uuid::new_v4();
    let new_user = user::create(
        &*conn,
        union_id,
        &params.username,
        &params.password,
        &params.invitation_code,
        GroupId::Normal as i64,
    )?;

    let url = String::from("/users/self");

    let jwt = bearer::encode(&config.jwt.secret, &new_user)?;

    let auth = Auth {
        state: AuthState::PASS,
        user: Some(new_user),
        jwt: Some(jwt),
    };

    Ok(Created(url, Some(Json(auth))))
}

#[derive(Serialize, Deserialize)]
pub struct SigninParams {
    username: String,
    password: String,
}

#[post("/signin", data = "<params>")]
pub fn signin(
    mut cookies: Cookies,
    config: State<Config>,
    db: State<Database>,
    client_addr: ClientAddr,
    params: Json<SigninParams>,
) -> Result<Json<Auth>, Error> {
    let conn = db.get_conn()?;

    let happened_time = Utc::now() - Duration::hours(1);
    let activities: Vec<SigninActivity> =
        audit::select(&*conn, &params.username, Signin, happened_time)?;
    let mut failed_signin_activities: Vec<SigninActivity> = vec![];
    for activity in activities {
        if !activity.details.is_succeed {
            failed_signin_activities.push(activity);
        }
    }
    if failed_signin_activities.len() >= 3 {
        Err(Error::Forbidden)
    } else {
        match user::auth(&*conn, &params.username, &params.password) {
            Ok(auth_user) => {
                let activity = SigninActivity {
                    client_addr: client_addr.0,
                    happened_time: Utc::now(),
                    activity_type: Signin,
                    details: SigninActivityDetails { is_succeed: true },
                };
                audit::create(&*conn, &params.username, activity)?;

                let mut cookie = Cookie::new("identity", auth_user.id.to_string());
                cookie.set_http_only(true);
                cookies.add_private(cookie);

                let preferences = preference::select(&*conn, auth_user.id)?;
                let totp_preference = preferences.iter().find(|preference| {
                    preference.key == PreferenceKey::SIGNIN_TOTP as i32
                        && preference.enabled == true
                });

                let auth = match totp_preference {
                    Some(_) => Auth {
                        state: AuthState::NEED_TOTP,
                        user: None,
                        jwt: None,
                    },
                    None => {
                        let jwt = bearer::encode(&config.jwt.secret, &auth_user)?;
                        Auth {
                            state: AuthState::PASS,
                            user: Some(auth_user),
                            jwt: Some(jwt),
                        }
                    }
                };
                Ok(Json(auth))
            }
            Err(model_err) => {
                let activity = SigninActivity {
                    client_addr: client_addr.0,
                    happened_time: Utc::now(),
                    activity_type: Signin,
                    details: SigninActivityDetails { is_succeed: false },
                };
                audit::create(&*conn, &params.username, activity)?;

                Err(Error::Model(model_err))
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TotpVerifyParams {
    code: u64,
}

#[post("/signin/totp", data = "<params>")]
pub fn verify_totp(
    db: State<Database>,
    mut cookies: Cookies,
    config: State<Config>,
    params: Json<TotpVerifyParams>,
) -> Result<Json<Auth>, Error> {
    match cookies.get_private("identity") {
        Some(identity) => {
            let conn = db.get_conn()?;
            match identity.value().parse::<i64>() {
                Ok(user_id) => {
                    let is_valid = totp::verify(&*conn, user_id, params.code)?;
                    if (is_valid) {
                        let auth_user = user::select_user(&*conn, user_id)?;
                        let jwt = bearer::encode(&config.jwt.secret, &auth_user)?;

                        cookies.remove_private(Cookie::named("identity"));

                        Ok(Json(Auth {
                            state: AuthState::PASS,
                            user: Some(auth_user),
                            jwt: Some(jwt),
                        }))
                    } else {
                        Err(Error::Params)
                    }
                }
                Err(_err) => Err(Error::Params),
            }
        }
        None => Err(Error::Params),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignoutResponse {}

#[post("/signout")]
pub fn signout() -> Result<Json<SignoutResponse>, Error> {
    Ok(Json(SignoutResponse {}))
}

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordParams {
    old_password: String,
    new_password: String,
}

#[post("/users/<user_id>/password", data = "<params>")]
pub fn change_password(
    config: State<Config>,
    db: State<Database>,
    client_addr: ClientAddr,
    user_id: i64,
    claims: Claims,
    params: Json<ChangePasswordParams>,
) -> Result<Json<()>, Error> {
    if claims.uid == user_id {
        let conn = db.get_conn()?;

        match user::change_password(&*conn, user_id, &params.old_password, &params.new_password) {
            Ok(()) => Ok(Json(())),
            Err(model_err) => Err(Error::Model(model_err)),
        }
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users?<group_id>&<paginator_params..>")]
pub fn select_users(
    config: State<Config>,
    db: State<Database>,
    client_addr: ClientAddr,
    permissions: Permissions,
    group_id: Option<i64>,
    paginator_params: Form<PaginatorParams>,
) -> Result<Json<ResourceCollection<User>>, Error> {
    if permissions.contains(ResourceType::User, ActionType::SELECT) {
        let conn = db.get_conn()?;

        let users = user::select_users(&*conn, group_id, &paginator_params)?;

        Ok(Json(users))
    } else {
        Err(Error::Forbidden)
    }
}
