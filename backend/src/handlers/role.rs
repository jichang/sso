use rocket::State;
use rocket_contrib::json::Json;

use super::super::models::role;
use super::super::models::role::Role;
use super::super::storage::Database;
use super::Error;

#[get("/roles")]
pub fn select_roles(db: State<Database>) -> Result<Json<Vec<Role>>, Error> {
    let conn = db.get_conn()?;
    let roles = role::select(&*conn)?;

    Ok(Json(roles))
}
