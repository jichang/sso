use rocket::State;
use rocket_contrib::Json;

use super::Error;
use super::super::models::role;
use super::super::models::role::Role;
use super::super::storage::Database;

#[get("/roles")]
fn query_roles(db: State<Database>) -> Result<Json<Vec<Role>>, Error> {
    let conn = db.get_conn()?;
    let roles = role::select(&*conn)?;

    Ok(Json(roles))
}
