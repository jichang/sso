use hex::ToHex;
use rand::{Rng, StdRng};
use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;
use redis::{Connection, RedisError, Commands};

#[derive(Debug)]
pub enum TokenError {
    Create(IoError),
    Redis(RedisError),
    Invalid,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::Create(ref err) => err.fmt(f),
            TokenError::Redis(ref err) => err.fmt(f),
            TokenError::Invalid => write!(f, "token is not valid"),
        }
    }
}

impl StdError for TokenError {
    fn description(&self) -> &str {
        match *self {
            TokenError::Create(_) => "can not send token",
            TokenError::Redis(_) => "can not access redis",
            TokenError::Invalid => "token is not valid",
        }
    }
}

impl From<RedisError> for TokenError {
    fn from(err: RedisError) -> TokenError {
        TokenError::Redis(err)
    }
}

impl From<IoError> for TokenError {
    fn from(err: IoError) -> TokenError {
        TokenError::Create(err)
    }
}

const TOKEN_SIZE: usize = 32;

pub fn create() -> Result<String, TokenError> {
    let mut token = [0u8; TOKEN_SIZE];
    let mut rng = StdRng::new()?;
    rng.fill_bytes(&mut token);

    Ok(token.to_hex())
}

pub fn store(redis_conn: &Connection, email_addr: &str, token: &str) -> Result<(), TokenError> {
    let expire = 5 * 60;
    let key = format!("token:email:{}", email_addr);
    let _: String = redis_conn.set_ex(&key, token, expire)?;

    Ok(())
}

pub fn verify(redis_conn: &Connection, email_addr: &str, token: &str) -> Result<(), TokenError> {
    let key = format!("token:email:{}", email_addr);
    let saved_token: Option<String> = redis_conn.get(&key)?;

    match saved_token {
        Some(saved_token) => {
            if saved_token == token {
                let _: () = redis_conn.del(&key)?;

                Ok(())
            } else {
                Err(TokenError::Invalid)
            }
        }
        None => Err(TokenError::Invalid),
    }
}


#[cfg(test)]
mod test {
    use redis;
    use super::{create, store, verify, TokenError};

    #[test]
    fn test_token_store_and_verify() {
        let client = redis::Client::open("redis://localhost/").unwrap();
        let conn = client.get_connection().unwrap();

        let token = create().unwrap();
        let email = "test@test.com";

        let _: () = store(&conn, &email, &token).unwrap();
        let res = verify(&conn, &email, &token).unwrap();
        assert_eq!(res, ());

        let wrong_token = "hello,this is wrong";
        let res = verify(&conn, &email, &wrong_token);
        assert_eq!(res.is_err(), true);

        match res {
            Err(err) => {
                match (err) {
                    TokenError::Create(_) => panic!(),
                    TokenError::Redis(_) => panic!(),
                    TokenError::Invalid => (),
                }
            }
            _ => panic!(),
        }
    }
}
