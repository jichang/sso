use chrono::Utc;
use redis::{Commands, Connection, RedisError};
use serde_json;

const RATELIMIT_LIMIT: i32 = 60 * 60;
const RATELIMIT_RESET: i64 = 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: i32,
    pub reset: i64,
    pub remaining: i32,
}

impl RateLimit {
    pub fn new() -> Self {
        let reset = Utc::now().timestamp() + RATELIMIT_RESET;
        RateLimit {
            limit: RATELIMIT_LIMIT,
            reset: reset,
            remaining: RATELIMIT_LIMIT,
        }
    }

    pub fn select(redis_conn: &Connection, identity: &str) -> Result<Self, RedisError> {
        let key = format!("ratelimit:identity:{}", identity);
        let result: Result<String, _> = redis_conn.get(key);
        match result {
            Ok(token) => Ok(serde_json::from_str(&token).unwrap_or(RateLimit::new())),
            Err(_err) => Ok(RateLimit::new()),
        }
    }

    pub fn update(
        redis_conn: &Connection,
        identity: &str,
        rate_limit: &RateLimit,
    ) -> Result<(), RedisError> {
        let key = format!("ratelimit:identity:{}", identity);
        let token = serde_json::to_string(&rate_limit).unwrap();
        let _: Result<String, _> = redis_conn.set_ex(key, token, RATELIMIT_RESET as usize);
        Ok(())
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        let reset = Utc::now().timestamp() + RATELIMIT_RESET;
        RateLimit {
            limit: RATELIMIT_LIMIT,
            reset: reset,
            remaining: RATELIMIT_LIMIT,
        }
    }
}
