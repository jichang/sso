use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::models::contact;
use super::super::models::contact::{Contact, ContactType};
use super::super::storage::Database;
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
    db: State<Database>,
    params: Json<CreateContactParams>,
    user_id: i64,
    claims: Claims,
) -> Result<Created<Json<Contact>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_contact = contact::create(&*pg_conn, user_id, params.type_id, &params.identity)?;

        let url = String::from("/contacts");
        Ok(Created(url, Some(Json(new_contact))))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/contacts")]
pub fn select_contacts(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    claims: Claims,
) -> Result<Json<Vec<Contact>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let contacts = contact::select(&*pg_conn, user_id)?;

        Ok(Json(contacts))
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
    claims: Claims,
) -> Result<Json<Contact>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_contact = contact::remove(&*pg_conn, contact_id, user_id)?;

        Ok(Json(removed_contact))
    } else {
        Err(Error::Privilege)
    }
}
