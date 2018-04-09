use std::fmt;
use std::convert::From;
use std::error::Error as StdError;
use url::ParseError;

pub mod crypto;
pub mod email;
pub mod username;
pub mod token;
pub mod contact;
pub mod role;
pub mod group;
pub mod user;
pub mod profile;
pub mod application;
pub mod authorization;
pub mod ticket;
pub mod ratelimit;

use self::username::Error as UsernameError;
use self::crypto::Error as CryptoError;
use postgres::error::Error as PgError;

#[derive(Debug)]
pub enum Error {
    Request(Box<StdError>),
    InvalidParam(String, Box<StdError>),
    Database(PgError),
    NotFound,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Request(ref err) => err.fmt(f),
            Error::InvalidParam(ref _field, ref err) => err.fmt(f),
            Error::Database(ref err) => err.fmt(f),
            Error::NotFound => write!(f, "Resource not found"),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Request(ref err) => err.description(),
            Error::InvalidParam(ref _field, ref err) => err.description(),
            Error::Database(ref err) => err.description(),
            Error::NotFound => "Resource not found",
            Error::Unknown => "Unknown",
        }
    }
}

impl From<PgError> for Error {
    fn from(err: PgError) -> Error {
        Error::Database(err)
    }
}

impl From<UsernameError> for Error {
    fn from(err: UsernameError) -> Error {
        Error::InvalidParam(String::from("username"), Box::new(err))
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::InvalidParam(String::from("url"), Box::new(err))
    }
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::InvalidParam(String::from("password"), Box::new(err))
    }
}
