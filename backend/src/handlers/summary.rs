use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::guards::permission::Permissions;
use super::super::models::summary;
use super::super::models::summary::Summary;
use super::super::storage::Database;
use super::Error;

#[get("/users/<user_id>/summary")]
pub fn select_summary(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    permissions: Permissions,
) -> Result<Json<Summary>, Error> {
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let user_summary = summary::select(&*pg_conn, user_id, &permissions)?;

        Ok(Json(user_summary))
    } else {
        Err(Error::Privilege)
    }
}
