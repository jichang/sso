use rocket::State;
use rocket_contrib::Json;

use hex::FromHex;

use super::super::common;
use super::super::config::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::models::application;
use super::super::models::application::{Application, Scope};
use super::super::models::ratelimit::RateLimit;
use super::super::storage::Database;
use super::Error;

const CLIENT_ID_LEN: usize = 64;
const CLIENT_SECRET_LEN: usize = 128;

#[derive(Serialize, Deserialize)]
pub struct CreateApplicationParams {
    name: String,
    website_uri: String,
    callback_uri: String,
}

#[post("/users/<user_id>/applications", data = "<params>")]
pub fn create_application(
    config: State<Config>,
    db: State<Database>,
    params: Json<CreateApplicationParams>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Application>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let client_id = common::gen_rand_bytes(CLIENT_ID_LEN)?;
        let client_secret = common::gen_rand_bytes(CLIENT_SECRET_LEN)?;
        let pg_conn = db.get_conn()?;
        let new_application = application::create(
            &*pg_conn,
            user_id,
            &params.name,
            &params.website_uri,
            &client_id,
            &client_secret,
            &params.callback_uri,
        )?;

        Ok(Json(new_application))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/applications")]
pub fn select_applications(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
    _rate_limit: RateLimit,
) -> Result<Json<Vec<Application>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let applications = application::select(&*pg_conn, user_id)?;

        Ok(Json(applications))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/applications/<application_id>")]
pub fn remove_application(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Application>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_application = application::remove(&*pg_conn, user_id, application_id)?;

        Ok(Json(removed_application))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateScopeParams {
    name: String,
    description: String,
}

#[post("/users/<user_id>/applications/<application_id>/scopes", data = "<params>")]
pub fn create_scope(
    config: State<Config>,
    db: State<Database>,
    params: Json<CreateScopeParams>,
    user_id: i64,
    application_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Scope>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_scope = application::create_scope(
            &*pg_conn,
            application_id,
            &params.name,
            &params.description,
        )?;

        Ok(Json(new_scope))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/applications/<application_id>/scopes")]
pub fn select_scopes(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Vec<Scope>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let scopes = application::select_scopes(&*pg_conn, application_id)?;

        Ok(Json(scopes))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/applications/<application_id>/scopes/<scope_id>")]
pub fn remove_scope(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    scope_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Scope>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_scope = application::remove_scope(&*pg_conn, application_id, scope_id)?;

        Ok(Json(removed_scope))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct SelectAppParams {
    client_id: String,
}

#[get("/applications?<params>")]
pub fn select_application(
    db: State<Database>,
    params: SelectAppParams,
) -> Result<Json<Application>, Error> {
    let client_id = Vec::<u8>::from_hex(params.client_id)?;
    let pg_conn = db.get_conn()?;
    let match_app = application::select_one(&*pg_conn, &client_id)?;

    Ok(Json(match_app))
}
