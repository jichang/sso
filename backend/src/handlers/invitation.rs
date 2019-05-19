use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::response::status::Created;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::guards::permission::Permissions;
use super::super::models::invitation;
use super::super::models::invitation::Invitation;
use super::super::models::resource::{ActionType, ResourceType};
use super::super::storage::Database;
use super::Error;

#[post("/users/<user_id>/invitations")]
pub fn create_invitation(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Created<Json<Invitation>>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::CREATE) {
        if claims.uid == user_id {
            let pg_conn = db.get_conn()?;
            let code: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

            let new_invitation = invitation::create(&*pg_conn, claims.role_id, user_id, &code)?;

            let url = String::from("/invitations");
            Ok(Created(url, Some(Json(new_invitation))))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[get("/users/<user_id>/invitations")]
pub fn select_invitations(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Json<Vec<Invitation>>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::SELECT) {
        if claims.uid == user_id {
            let pg_conn = db.get_conn()?;
            let invitations = invitation::select(&*pg_conn, user_id)?;

            Ok(Json(invitations))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}

#[delete("/users/<user_id>/invitations/<invitation_id>")]
pub fn remove_invitation(
    db: State<Database>,
    user_id: i64,
    invitation_id: i64,
    claims: Claims,
    permissins: Permissions,
) -> Result<Json<Invitation>, Error> {
    if permissins.contains(ResourceType::Application, ActionType::DELETE) {
        if claims.uid == user_id {
            let pg_conn = db.get_conn()?;
            let removed_invitation = invitation::remove(&*pg_conn, invitation_id, user_id)?;

            Ok(Json(removed_invitation))
        } else {
            Err(Error::Privilege)
        }
    } else {
        Err(Error::Forbidden)
    }
}
