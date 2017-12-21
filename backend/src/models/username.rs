use std::fmt;
use std::str;
use std::error::Error as StdError;
use postgres::types::{IsNull, Type, ToSql, FromSql, VARCHAR};

const MIN_LEN: usize = 5;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    MinLen(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MinLen(_len) => "username is smaller then minimum length",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Username {
    pub value: String,
}

impl Username {
    pub fn new(username: &str) -> Result<Username, Error> {
        if username.len() < MIN_LEN {
            return Err(Error::MinLen(MIN_LEN));
        }

        Ok(Username { value: username.to_string() })
    }
}

impl FromSql for Username {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Self, Box<StdError + Sync + Send>> {
        let username = str::from_utf8(raw)?;

        Ok(Username { value: username.to_owned() })
    }

    accepts!(VARCHAR);
}

impl ToSql for Username {
    fn to_sql(&self, _: &Type, w: &mut Vec<u8>) -> Result<IsNull, Box<StdError + Sync + Send>> {
        w.extend_from_slice(self.value.as_bytes());
        Ok(IsNull::No)
    }

    accepts!(VARCHAR);
    to_sql_checked!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_empty_username() {
        let username = Username::new("");
        assert!(username.is_err());
    }
}
