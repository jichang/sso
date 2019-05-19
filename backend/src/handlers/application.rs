use rocket::request::Form;
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use hex::FromHex;

use super::super::common;
use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::guards::permission::Permissions;
use super::super::models::application;
use super::super::models::application::{Application, Scope};
use super::super::models::resource::{ActionType, ResourceType};
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
    db: State<Database>,
    params: Json<CreateApplicationParams>,
    user_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Created<Json<Application>>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::CREATE) {
        if claims.uid == user_id {
            let client_id = common::gen_rand_bytes(CLIENT_ID_LEN)?;
            let client_secret = common::gen_rand_bytes(CLIENT_SECRET_LEN)?;
            let pg_conn = db.get_conn()?;
            let new_application = application::create(
                &*pg_conn,
                claims.role_id,
                user_id,
                &params.name,
                &params.website_uri,
                &client_id,
                &client_secret,
                &params.callback_uri,
            )?;

            let url = String::from("/applications");

            Ok(Created(url, Some(Json(new_application))))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[get("/users/<user_id>/applications")]
pub fn select_applications(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Json<Vec<Application>>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::SELECT) {
        if claims.uid == user_id {
            let pg_conn = db.get_conn()?;
            let applications = application::select(&*pg_conn, user_id)?;

            Ok(Json(applications))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[delete("/users/<user_id>/applications/<application_id>")]
pub fn remove_application(
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Json<Application>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::DELETE) {
        if claims.uid == user_id {
            let pg_conn = db.get_conn()?;
            let removed_application = application::remove(&*pg_conn, user_id, application_id)?;

            Ok(Json(removed_application))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateScopeParams {
    name: String,
    description: String,
}

#[post(
    "/users/<user_id>/applications/<application_id>/scopes",
    data = "<params>"
)]
pub fn create_scope(
    db: State<Database>,
    params: Json<CreateScopeParams>,
    user_id: i64,
    application_id: i64,
    claims: Claims,
) -> Result<Created<Json<Scope>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_scope = application::create_scope(
            &*pg_conn,
            application_id,
            &params.name,
            &params.description,
        )?;

        let url = String::from("/scopes");

        Ok(Created(url, Some(Json(new_scope))))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/applications/<application_id>/scopes")]
pub fn select_scopes(
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    claims: Claims,
) -> Result<Json<Vec<Scope>>, Error> {
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
    db: State<Database>,
    user_id: i64,
    application_id: i64,
    scope_id: i64,
    claims: Claims,
) -> Result<Json<Scope>, Error> {
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

#[get("/applications?<params..>")]
pub fn select_application(
    db: State<Database>,
    params: Form<SelectAppParams>,
) -> Result<Json<Application>, Error> {
    let client_id = Vec::<u8>::from_hex(&params.client_id)?;
    let pg_conn = db.get_conn()?;
    let match_app = application::select_one(&*pg_conn, &client_id)?;

    Ok(Json(match_app))
}
