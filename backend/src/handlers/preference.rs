use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::models::preference;
use super::super::models::preference::Preference;
use super::super::storage::Database;
use super::Error;

#[post("/users/<user_id>/preferences", data = "<params>")]
pub fn create_preference(
    db: State<Database>,
    params: Json<Preference>,
    user_id: i64,
    claims: Claims,
) -> Result<Created<Json<Preference>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_preference = preference::create(&*pg_conn, user_id, &params)?;

        let url = String::from("/preferences");
        Ok(Created(url, Some(Json(new_preference))))
    } else {
        Err(Error::Privilege)
    }
}

#[get("/users/<user_id>/preferences")]
pub fn select_preferences(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
) -> Result<Json<Vec<Preference>>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let preferences = preference::select(&*pg_conn, user_id)?;

        Ok(Json(preferences))
    } else {
        Err(Error::Privilege)
    }
}
