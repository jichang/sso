use rocket::State;
use rocket_contrib::json::Json;

use super::super::guards::permission::Permissions;
use super::super::models::permission;
use super::super::models::permission::{ActionType, Permission, ResourceType};
use super::super::storage::Database;
use super::Error;

#[get("/permissions")]
pub fn select_permissions(
    db: State<Database>,
    permissions: Permissions,
) -> Result<Json<Vec<Permission>>, Error> {
    if permissions.contains(ResourceType::Role, ActionType::SELECT) {
        let conn = db.get_conn()?;
        let permissions = permission::select_all(&*conn)?;

        Ok(Json(permissions))
    } else {
        Err(Error::Forbidden)
    }
}
