use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

use super::super::models::permission;
use super::super::models::permission::Permission;
use super::super::storage::Database;
use super::bearer::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    permissions: Vec<Permission>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Permissions {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Permissions, ()> {
        let claims = request.guard::<Claims>()?;
        let database = request.guard::<State<Database>>()?;
        match database.get_conn() {
            Ok(pg_conn) => match permission::select(&*pg_conn, claims.role_id) {
                Ok(permissions) => Outcome::Success(Permissions {
                    permissions: permissions,
                }),
                Err(_err) => Outcome::Failure((Status::InternalServerError, ())),
            },
            Err(_err) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}
