use chrono::{Duration, Utc};
use rocket::http::{Cookie, Cookies};
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::client_addr::ClientAddr;
use super::super::models::audit;
use super::super::models::audit::{Signin, SigninActivity, SigninActivityDetails};
use super::super::models::group::GroupId;
use super::super::models::user;
use super::super::models::user::User;
use super::super::storage::Database;
use super::Error;

pub const UNION_ID_LEN: usize = 32;

#[derive(Serialize, Deserialize)]
pub struct Auth {
    user: User,
    jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupParams {
    username: String,
    password: String,
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
        GroupId::Normal as i64,
    )?;
    let url = String::from("/users/self");

    cookies.add_private(Cookie::new("identity", new_user.id.to_string()));

    let jwt = bearer::encode(&config.jwt.secret, &new_user)?;

    let auth = Auth {
        user: new_user,
        jwt: jwt,
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
                let jwt = bearer::encode(&config.jwt.secret, &auth_user)?;
                let activity = SigninActivity {
                    client_addr: client_addr.0,
                    happened_time: Utc::now(),
                    activity_type: Signin,
                    details: SigninActivityDetails { is_succeed: true },
                };
                audit::create(&*conn, &params.username, activity)?;

                cookies.add_private(Cookie::new("identity", auth_user.id.to_string()));

                let auth = Auth {
                    user: auth_user,
                    jwt: jwt,
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
pub struct SignoutResponse {}

#[post("/signout")]
pub fn signout() -> Result<Json<SignoutResponse>, Error> {
    Ok(Json(SignoutResponse {}))
}
