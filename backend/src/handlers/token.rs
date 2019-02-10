use rocket::request::Form;
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use hex;
use redis::Commands;

use super::super::common;
use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::models::contact;
use super::super::models::mailer;
use super::super::storage::{Cache, Database};
use super::Error;

const TOKEN_SIZE: usize = 64;
const TOKEN_EXPIRE_DURATION: usize = 5 * 60;

#[derive(Serialize, Deserialize)]
pub struct CreateTokenParams {
    target_id: i64,
    target_type: String,
    target_identity: String,
    action: String,
}

#[post("/users/<user_id>/tokens", data = "<params>")]
pub fn create_token(
    config: State<Config>,
    cache: State<Cache>,
    user_id: i64,
    params: Json<CreateTokenParams>,
    bearer: AuthorizationBearer,
) -> Result<Created<Json<()>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let redis_conn = cache.get_conn()?;
        let token = common::gen_rand_bytes(TOKEN_SIZE)?;
        let key = format!(
            "token:{}:{}:{}",
            params.target_id, params.target_type, params.action
        );
        let _: String = redis_conn.set_ex(&key, hex::encode(&token), TOKEN_EXPIRE_DURATION)?;

        let _ = mailer::send_token(
            &config,
            params.target_id,
            &params.target_identity,
            &hex::encode(&token),
        )?;

        let url = String::from("/tokens");
        Ok(Created(url, Some(Json(()))))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(FromForm)]
pub struct DeleteTokenParams {
    target_id: i64,
    target_type: String,
    action: String,
    token: String,
}

#[delete("/users/<user_id>/tokens?<params..>")]
pub fn delete_token(
    config: State<Config>,
    db: State<Database>,
    cache: State<Cache>,
    user_id: i64,
    params: Form<DeleteTokenParams>,
    bearer: AuthorizationBearer,
) -> Result<Json<()>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let redis_conn = cache.get_conn()?;
        let key = format!(
            "token:{}:{}:{}",
            params.target_id, params.target_type, params.action
        );
        let token_result: Option<String> = redis_conn.get(&key)?;

        match token_result {
            Some(token) => {
                if token == params.token {
                    let pg_conn = db.get_conn()?;
                    let _ = contact::verify(&*pg_conn, user_id, params.target_id)?;
                    let _: () = redis_conn.del(&key)?;

                    Ok(Json(()))
                } else {
                    Err(Error::Params)
                }
            }
            None => Err(Error::Params),
        }
    } else {
        Err(Error::Forbidden)
    }
}
