use rocket::State;
use rocket_contrib::json::Json;

use hex;
use redis::Commands;

use super::super::common;
use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::models::contact;
use super::super::models::contact::{Contact, ContactType};
use super::super::storage::{Cache, Database};
use super::mailer;
use super::Error;

#[get("/contacts/types")]
pub fn select_types(db: State<Database>) -> Result<Json<Vec<ContactType>>, Error> {
    let conn = db.get_conn()?;
    let types = contact::select_types(&*conn)?;

    Ok(Json(types))
}

const VERIFY_TOKEN_SIZE: usize = 64;

#[derive(Serialize, Deserialize)]
pub struct CreateContactParams {
    identity: String,
    type_id: i32,
}

#[post("/users/<user_id>/contacts", data = "<params>")]
pub fn create_contact(
    config: State<Config>,
    cache: State<Cache>,
    db: State<Database>,
    params: Json<CreateContactParams>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Contact>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_contact = contact::create(&*pg_conn, user_id, params.type_id, &params.identity)?;

        Ok(Json(new_contact))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/contacts")]
pub fn select_contacts(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Vec<Contact>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let contacts = contact::select(&*pg_conn, user_id)?;

        Ok(Json(contacts))
    } else {
        Err(Error::Privilege)
    }
}

#[post("/users/<user_id>/contacts/<contact_id>/verifications")]
pub fn send_verify_token(
    config: State<Config>,
    db: State<Database>,
    cache: State<Cache>,
    user_id: i64,
    contact_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Contact>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let match_contact = contact::select_one(&*pg_conn, user_id, contact_id)?;

        let redis_conn = cache.get_conn()?;
        let verify_token = common::gen_rand_bytes(VERIFY_TOKEN_SIZE)?;
        let expire = 5 * 60;
        let key = format!("token:verify:contact:{}", match_contact.id);
        let _: String = redis_conn.set_ex(&key, hex::encode(&verify_token), expire)?;

        let _ = mailer::send_token(
            &config,
            match_contact.id,
            &match_contact.identity,
            &hex::encode(&verify_token),
        );

        Ok(Json(match_contact))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/contacts/<contact_id>")]
pub fn remove_contact(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    contact_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Contact>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_contact = contact::remove(&*pg_conn, contact_id, user_id)?;

        Ok(Json(removed_contact))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateContactParams {
    token: String,
}

#[post("/users/<user_id>/contacts/<contact_id>", data = "<params>")]
pub fn verify_contact(
    config: State<Config>,
    db: State<Database>,
    cache: State<Cache>,
    params: Json<UpdateContactParams>,
    user_id: i64,
    contact_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Contact>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let redis_conn = cache.get_conn()?;
        let key = format!("token:verify:contact:{}", contact_id);
        let saved_token: Option<String> = redis_conn.get(&key)?;

        match saved_token {
            Some(saved_token) => {
                if saved_token == params.token {
                    let _: () = redis_conn.del(&key)?;

                    let conn = db.get_conn()?;
                    let new_contact = contact::verify(&*conn, user_id, contact_id)?;

                    Ok(Json(new_contact))
                } else {
                    Err(Error::Privilege)
                }
            }
            None => Err(Error::Privilege),
        }
    } else {
        Err(Error::Privilege)
    }
}
