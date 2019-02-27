use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::guards::permission::Permissions;
use super::super::models::group;
use super::super::models::group::Group;
use super::super::models::permission::{ActionType, ResourceType};
use super::super::models::user::User;
use super::super::models::PaginatorParams;
use super::super::models::ResourceCollection;
use super::super::storage::Database;
use super::Error;

#[get("/groups")]
pub fn select_groups(
    db: State<Database>,
    permissions: Permissions,
) -> Result<Json<Vec<Group>>, Error> {
    if permissions.contains(ResourceType::Group, ActionType::SELECT) {
        let conn = db.get_conn()?;
        let groups = group::select(&*conn)?;

        Ok(Json(groups))
    } else {
        Err(Error::Forbidden)
    }
}

#[get("/groups/<group_id>/users?<params..>")]
pub fn select_users(
    db: State<Database>,
    permissions: Permissions,
    group_id: i64,
    params: Form<PaginatorParams>,
) -> Result<Json<ResourceCollection<User>>, Error> {
    if permissions.contains(ResourceType::GroupUser, ActionType::SELECT) {
        let conn = db.get_conn()?;
        let users = group::select_users(&*conn, group_id, &params)?;

        Ok(Json(users))
    } else {
        Err(Error::Forbidden)
    }
}
