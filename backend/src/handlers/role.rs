use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::guards::permission::Permissions;
use super::super::models::permission::Permission;
use super::super::models::resource::{ActionType, ResourceType};
use super::super::models::role;
use super::super::models::role::Role;
use super::super::models::user::User;
use super::super::storage::Database;
use super::Error;

#[get("/roles")]
pub fn select_roles(
    db: State<Database>,
    permissions: Permissions,
) -> Result<Json<Vec<Role>>, Error> {
    if permissions.contains(ResourceType::Role, ActionType::SELECT) {
        let conn = db.get_conn()?;
        let roles = role::select(&*conn)?;

        Ok(Json(roles))
    } else {
        Err(Error::Forbidden)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreatePermissionParams {
    resource_type: i32,
    action_type: i32,
}

#[post("/roles/<role_id>/permissions", data = "<params>")]
pub fn create_permission(
    db: State<Database>,
    permissions: Permissions,
    role_id: i32,
    params: Json<CreatePermissionParams>,
) -> Result<Json<Permission>, Error> {
    if permissions.contains(ResourceType::RolePermission, ActionType::CREATE) {
        let conn = db.get_conn()?;
        let resource_type = ResourceType::from_i32(params.resource_type);
        let action_type = ActionType::from_i32(params.action_type);
        let permission = role::create_permission(&*conn, role_id, resource_type, action_type)?;

        Ok(Json(permission))
    } else {
        Err(Error::Forbidden)
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct DeletePermissionParams {
    resource_type: i32,
    action_type: i32,
}

#[delete("/roles/<role_id>/permissions?<params..>")]
pub fn remove_permission(
    db: State<Database>,
    permissions: Permissions,
    role_id: i32,
    params: Form<DeletePermissionParams>,
) -> Result<Json<Permission>, Error> {
    if permissions.contains(ResourceType::RolePermission, ActionType::DELETE) {
        let conn = db.get_conn()?;
        let resource_type = ResourceType::from_i32(params.resource_type);
        let action_type = ActionType::from_i32(params.action_type);
        let permission = role::remove_permission(&*conn, role_id, resource_type, action_type)?;

        Ok(Json(permission))
    } else {
        Err(Error::Forbidden)
    }
}
