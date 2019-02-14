use rocket::request::Form;
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use hex;
use hex::FromHex;
use redis::Commands;
use url::Url;
use uuid;
use uuid::Uuid;

use super::super::common;
use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::models::application;
use super::super::models::authorization;
use super::super::models::authorization::{Authorization, AuthorizationPreview};
use super::super::storage::{Cache, Database};
use super::Error;

const AUTH_CODE_SIZE: usize = 64;

#[derive(Serialize, Deserialize)]
pub struct CreateAuthorizationParams {
    server_id: String,
    client_id: String,
    scope_name: String,
    redirect_uri: String,
    response_type: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
pub struct Credientials {
    code: String,
    state: String,
}

#[post("/users/<user_id>/authorizations", data = "<params>")]
pub fn create_authorization(
    config: State<Config>,
    cache: State<Cache>,
    db: State<Database>,
    user_id: i64,
    params: Json<CreateAuthorizationParams>,
    claims: Claims,
) -> Result<Created<Json<Credientials>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let name = user_id.to_string() + &params.client_id + &params.server_id;
        let open_id = Uuid::new_v5(&uuid::NAMESPACE_DNS, &name);
        let server_id = Vec::<u8>::from_hex(&params.server_id)?;
        let client_id = Vec::<u8>::from_hex(&params.client_id)?;
        let client_application = application::select_one(&*pg_conn, &client_id)?;

        let callback_uri = Url::parse(&client_application.callback_uri)?;
        let redirect_uri = Url::parse(&params.redirect_uri)?;
        if callback_uri.origin() != redirect_uri.origin()
            || callback_uri.path() != redirect_uri.path()
        {
            return Err(Error::Params);
        }

        let new_authorization = authorization::create(
            &*pg_conn,
            user_id,
            &open_id,
            &server_id,
            &client_id,
            &params.scope_name,
        )?;

        let redis_conn = cache.get_conn()?;
        let code = common::gen_rand_bytes(AUTH_CODE_SIZE)?;
        let expire = 5 * 60;
        let key = format!("oauth:code:{}", hex::encode(&code));
        let _: String = redis_conn.set_ex(&key, new_authorization.id, expire)?;

        let credientials = Credientials {
            code: hex::encode(&code),
            state: params.state.clone(),
        };

        let url = String::from("/authorizations");

        Ok(Created(url, Some(Json(credientials))))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/authorizations")]
pub fn select_authorizations(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    claims: Claims,
) -> Result<Json<Vec<Authorization>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let authorizations = authorization::select(&*pg_conn, user_id)?;

        Ok(Json(authorizations))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/authorizations/<authorization_id>")]
pub fn remove_authorization(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    authorization_id: i64,
    claims: Claims,
) -> Result<Json<Authorization>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_authorization = authorization::remove(&*pg_conn, user_id, authorization_id)?;

        Ok(Json(removed_authorization))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct SelectAuthorizationParams {
    server_id: String,
    client_id: String,
    scope_name: String,
}

#[get("/authorizations/preview?<params..>")]
pub fn preview_authorization(
    db: State<Database>,
    params: Form<SelectAuthorizationParams>,
) -> Result<Json<AuthorizationPreview>, Error> {
    let server_id = Vec::<u8>::from_hex(&params.server_id)?;
    let client_id = Vec::<u8>::from_hex(&params.client_id)?;
    let pg_conn = db.get_conn()?;
    let match_authorization =
        authorization::preview(&*pg_conn, &server_id, &client_id, &params.scope_name)?;

    Ok(Json(match_authorization))
}
