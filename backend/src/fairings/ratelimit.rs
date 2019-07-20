use rocket::http::Header;
use rocket::outcome::Outcome;
use rocket::{Request, Response, State};

use super::super::models::ratelimit::RateLimit;
use super::super::storage::Cache;

pub fn on_response(request: &Request, response: &mut Response) {
    let rate_limit = match request.cookies().get_private("identity") {
        Some(identity) => {
            let result = request.guard::<State<Cache>>();
            match result {
                Outcome::Success(cache) => match cache.get_conn() {
                    Ok(mut redis_conn_manager) => {
                        match RateLimit::select(&mut *redis_conn_manager, identity.value()) {
                            Ok(rate_limit) => rate_limit,
                            Err(_) => RateLimit::default(),
                        }
                    }
                    _ => RateLimit::default(),
                },
                _ => RateLimit::default(),
            }
        }
        _ => RateLimit::default(),
    };

    response.set_header(Header::new(
        "X-RateLimit-Limit",
        rate_limit.limit.to_string(),
    ));
    response.set_header(Header::new(
        "X-RateLimit-Reset",
        rate_limit.reset.to_string(),
    ));
    response.set_header(Header::new(
        "X-RateLimit-Remaining",
        rate_limit.remaining.to_string(),
    ));
}
