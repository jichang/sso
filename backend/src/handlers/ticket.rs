use rocket::State;
use rocket_contrib::Json;

use md5;
use hex::{FromHex};
use redis::Commands;

use super::Error;
use super::super::common;
use super::super::models::authorization;
use super::super::models::ticket;
use super::super::models::ticket::Ticket;
use super::super::storage::{Database, Cache};

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
) -> Result<Json<Ticket>, Error> {
    let redis_conn = cache.get_conn()?;
    let key = format!("oauth:code:{}", params.code);
    let code_value: Option<i64> = redis_conn.get(&key)?;

    match code_value {
        Some(authorization_id) => {
            let conn = db.get_conn()?;
            let match_authorization = authorization::select_one(&*conn, authorization_id)?;

            if match_authorization.client_app.client_secret.value() == params.client_secret &&
                match_authorization.client_app.client_id == params.client_id
            {
                let _: () = redis_conn.del(&key)?;

                let user_id = match_authorization.user_id.to_string();
                let digest = &md5::compute(user_id);
                let open_id = String::from_utf8(digest.to_vec())?;
                let access_token = common::gen_rand_bytes(ACCESS_TOKEN_SIZE)?;
                let refresh_token = common::gen_rand_bytes(REFRESH_TOKEN_SIZE)?;

                let new_ticket = ticket::create(
                    &*conn,
                    authorization_id,
                    &open_id,
                    &access_token,
                    &refresh_token,
                )?;

                Ok(Json(new_ticket))
            } else {
                Err(Error::Privilege)
            }
        }
        None => Err(Error::Privilege),
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTicketParams {
    open_id: String,
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

    let new_ticket = ticket::update(&*conn, &params.open_id, &access_token, &refresh_token)?;

    Ok(Json(new_ticket))
}
