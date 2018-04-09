use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::response::status::Created;
use rocket_contrib::Json;
use uuid::Uuid;

use super::super::config::Config;
use super::super::guards::bearer;
use super::super::models::group::GroupId;
use super::super::models::user;
use super::super::models::user::User;
use super::super::storage::Database;
use super::Error;

pub const UNION_ID_LEN: usize = 32;

#[derive(Serialize, Deserialize)]
struct Auth {
    user: User,
    jwt: String,
}

#[derive(Serialize, Deserialize)]
struct SignupParams {
    username: String,
    password: String,
}

#[post("/signup", data = "<params>")]
fn signup(
    mut cookies: Cookies,
    config: State<Config>,
    db: State<Database>,
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
struct SigninParams {
    username: String,
    password: String,
}

#[post("/signin", data = "<params>")]
fn signin(
    mut cookies: Cookies,
    config: State<Config>,
    db: State<Database>,
    params: Json<SigninParams>,
) -> Result<Json<Auth>, Error> {
    let conn = db.get_conn()?;
    let auth_user = user::auth(&*conn, &params.username, &params.password)?;
    let jwt = bearer::encode(&config.jwt.secret, &auth_user)?;

    cookies.add_private(Cookie::new("identity", auth_user.id.to_string()));

    let auth = Auth {
        user: auth_user,
        jwt: jwt,
    };

    Ok(Json(auth))
}

#[derive(Serialize, Deserialize)]
struct SignoutResponse {}

#[post("/signout")]
fn signout() -> Result<Json<SignoutResponse>, Error> {
    Ok(Json(SignoutResponse {}))
}
