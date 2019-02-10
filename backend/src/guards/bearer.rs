use chrono::prelude::*;
use jwt;
use jwt::errors::Error as JwtError;
use jwt::{Header, Validation};

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use super::super::models::user::User;

const CLAIMS_EXPIRE: i64 = 60 * 60 * 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    exp: i64,
    pub uid: i64,
    pub role: i32,
    pub status: i32,
}

pub fn encode(secret: &str, user: &User) -> Result<String, JwtError> {
    let claims = Claims {
        iss: "sso.feblr.org".to_string(),
        exp: Utc::now().timestamp() + CLAIMS_EXPIRE,
        uid: user.id,
        role: user.role.id,
        status: user.status,
    };

    let token = jwt::encode(&Header::default(), &claims, secret.as_ref())?;

    Ok(token)
}

pub fn decode(secret: &str, token: &str) -> Result<Claims, JwtError> {
    let data = jwt::decode::<Claims>(&token, secret.as_ref(), &Validation::default())?;

    Ok(data.claims)
}

pub struct AuthorizationBearer(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthorizationBearer {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthorizationBearer, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let key = keys[0];
        let parts: Vec<_> = key.split(" ").collect();
        if parts.len() != 2 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let bearer = parts[1];
        return Outcome::Success(AuthorizationBearer(bearer.to_string()));
    }
}
