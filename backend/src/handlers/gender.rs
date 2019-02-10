use rocket::State;
use rocket_contrib::json::Json;

use super::super::models::profile;
use super::super::models::profile::Gender;
use super::super::storage::Database;
use super::Error;

#[get("/genders")]
pub fn select_genders(db: State<Database>) -> Result<Json<Vec<Gender>>, Error> {
    let conn = db.get_conn()?;
    let genders = profile::select_genders(&*conn)?;

    Ok(Json(genders))
}
