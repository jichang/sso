use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;
use std::convert::From;
use std::error::Error as StdError;
use std::fmt;
use url::ParseError;

pub mod application;
pub mod audit;
pub mod authorization;
pub mod contact;
pub mod crypto;
pub mod email;
pub mod group;
pub mod invitation;
pub mod mailer;
pub mod permission;
pub mod preference;
pub mod profile;
pub mod quota;
pub mod ratelimit;
pub mod resource;
pub mod role;
pub mod summary;
pub mod ticket;
pub mod token;
pub mod totp;
pub mod user;
pub mod username;

use self::crypto::Error as CryptoError;
use self::mailer::MailerError;
use self::username::Error as UsernameError;
use postgres::error::Error as PgError;
use rocket::request::Form;

#[derive(Debug)]
pub enum Error {
    Request(Box<StdError>),
    InvalidParam(String, Box<StdError>),
    Database(PgError),
    Mailer(MailerError),
    Forbidden,
    NotFound,
    QuotaLimit,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Request(ref err) => err.fmt(f),
            Error::InvalidParam(ref _field, ref err) => err.fmt(f),
            Error::Database(ref err) => err.fmt(f),
            Error::Mailer(ref err) => err.fmt(f),
            Error::Forbidden => write!(f, "forbidden action"),
            Error::NotFound => write!(f, "Resource not found"),
            Error::QuotaLimit => write!(f, "Quota limit"),
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
            Error::Mailer(ref err) => err.description(),
            Error::Forbidden => "Forbidden action",
            Error::NotFound => "Resource not found",
            Error::QuotaLimit => "Quota limit",
            Error::Unknown => "Unknown",
        }
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Unknown
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

impl From<MailerError> for Error {
    fn from(err: MailerError) -> Error {
        Error::Mailer(err)
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct PaginatorParams {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize)]
pub struct ResourceCollection<T: Serialize> {
    total: i64,
    items: Vec<T>,
}
