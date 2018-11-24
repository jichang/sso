use chrono::{DateTime, Utc};
use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::models::profile;
use super::super::models::profile::Profile;
use super::super::storage::Database;
use super::Error;
use rocket::response::status::Created;

#[derive(Serialize, Deserialize)]
pub struct CreateProfileParams {
    name: String,
    gender_id: i32,
    birthday: DateTime<Utc>,
    introduction: String,
}

#[post("/users/<user_id>/profile", data = "<params>")]
pub fn create_profile(
    config: State<Config>,
    db: State<Database>,
    params: Json<CreateProfileParams>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Created<Json<Profile>>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_profile = profile::create(
            &*pg_conn,
            user_id,
            params.gender_id,
            &params.name,
            &params.birthday,
            &params.introduction,
        )?;

        let url = String::from("/applications");
        Ok(Created(url, Some(Json(new_profile))))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/profile")]
pub fn select_profile(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Profile>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_profile = profile::select(&*pg_conn, user_id)?;

        Ok(Json(new_profile))
    } else {
        Err(Error::Privilege)
    }
}

#[delete("/users/<user_id>/profile")]
pub fn remove_profile(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Profile>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let removed_profile = profile::remove(&*pg_conn, user_id)?;

        Ok(Json(removed_profile))
    } else {
        Err(Error::Privilege)
    }
}
