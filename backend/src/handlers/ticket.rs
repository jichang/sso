use rocket::State;
use rocket_contrib::json::Json;

use hex::FromHex;
use redis::Commands;
use rocket::response::status::Created;

use super::super::common;
use super::super::models::authorization;
use super::super::models::ticket;
use super::super::models::ticket::Ticket;
use super::super::storage::{Cache, Database};
use super::Error;

const ACCESS_TOKEN_SIZE: usize = 64;
const REFRESH_TOKEN_SIZE: usize = 64;

#[derive(Serialize, Deserialize)]
pub struct CreateTicketParams {
    client_id: String,
    client_secret: String,
    code: String,
}

#[post("/tickets", data = "<params>")]
pub fn create_ticket(
    db: State<Database>,
    cache: State<Cache>,
    params: Json<CreateTicketParams>,
) -> Result<Created<Json<Ticket>>, Error> {
    let redis_conn = cache.get_conn()?;
    let key = format!("oauth:code:{}", params.code);
    let code_value: Option<i64> = redis_conn.get(&key)?;

    match code_value {
        Some(authorization_id) => {
            let conn = db.get_conn()?;
            let client_id = Vec::<u8>::from_hex(&params.client_id)?;
            let client_secret = Vec::<u8>::from_hex(&params.client_secret)?;
            let open_id =
                authorization::verify(&*conn, authorization_id, &client_id, &client_secret)?;

            let _: () = redis_conn.del(&key)?;

            let access_token = common::gen_rand_bytes(ACCESS_TOKEN_SIZE)?;
            let refresh_token = common::gen_rand_bytes(REFRESH_TOKEN_SIZE)?;

            let new_ticket = ticket::create(
                &*conn,
                authorization_id,
                &open_id,
                &access_token,
                &refresh_token,
            )?;

            let url = String::from("/tickets");
            Ok(Created(url, Some(Json(new_ticket))))
        }
        None => Err(Error::Privilege),
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTicketParams {
    client_id: String,
    refresh_token: String,
}

#[put("/tickets", data = "<params>")]
pub fn update_ticket(
    db: State<Database>,
    params: Json<UpdateTicketParams>,
) -> Result<Json<Ticket>, Error> {
    let conn = db.get_conn()?;
    let refresh_token = Vec::<u8>::from_hex(&params.refresh_token)?;
    let access_token = common::gen_rand_bytes(ACCESS_TOKEN_SIZE)?;

    let new_ticket = ticket::update(&*conn, &params.client_id, &access_token, &refresh_token)?;

    Ok(Json(new_ticket))
}
