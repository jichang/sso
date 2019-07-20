use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

use super::super::models::ratelimit::RateLimit;
use super::super::storage::Cache;

impl<'a, 'r> FromRequest<'a, 'r> for RateLimit {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RateLimit, ()> {
        match request.cookies().get_private("identity") {
            Some(identity) => {
                let cache = request.guard::<State<Cache>>()?;
                match cache.get_conn() {
                    Ok(mut redis_conn_manager) => {
                        match RateLimit::select(&mut *redis_conn_manager, identity.value()) {
                            Ok(rate_limit) => {
                                if rate_limit.remaining <= 0 {
                                    Outcome::Failure((Status::TooManyRequests, ()))
                                } else {
                                    let rate_limit = RateLimit {
                                        limit: rate_limit.limit,
                                        reset: rate_limit.reset,
                                        remaining: rate_limit.remaining - 1,
                                    };
                                    let _ = RateLimit::update(
                                        &mut *redis_conn_manager,
                                        identity.value(),
                                        &rate_limit,
                                    );

                                    Outcome::Success(rate_limit)
                                }
                            }
                            Err(_err) => Outcome::Failure((Status::InternalServerError, ())),
                        }
                    }
                    Err(_err) => Outcome::Failure((Status::InternalServerError, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
