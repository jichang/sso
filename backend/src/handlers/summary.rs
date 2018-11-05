use rocket::State;
use rocket_contrib::json::Json;

use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::AuthorizationBearer;
use super::super::models::summary;
use super::super::models::summary::Summary;
use super::super::storage::Database;
use super::Error;

#[get("/users/<user_id>/summary")]
pub fn select_summary(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    bearer: AuthorizationBearer,
) -> Result<Json<Summary>, Error> {
    let claims = bearer::decode(&config.jwt.secret, bearer.0.as_str())?;
    if claims.uid == user_id {
        let pg_conn = db.get_conn()?;
        let new_summary = summary::select(&*pg_conn, user_id)?;

        Ok(Json(new_summary))
    } else {
        Err(Error::Privilege)
    }
}
