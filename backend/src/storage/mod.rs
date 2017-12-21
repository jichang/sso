use std::fmt;
use std::convert::From;
use std::error::Error as StdError;

use r2d2::{Config, Pool, PooledConnection, ManageConnection, GetTimeout, InitializationError};
use r2d2_postgres::PostgresConnectionManager;
use r2d2_redis::RedisConnectionManager;

#[derive(Debug)]
pub enum Error {
    PoolInit(InitializationError),
    PoolConn(GetTimeout),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PoolInit(ref err) => write!(f, "Storage pool init error: {}", err),
            Error::PoolConn(ref err) => write!(f, "Storage pool connect error: {}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PoolInit(_) => "storage pool init error",
            Error::PoolConn(_) => "storage pool connect error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::PoolInit(ref err) => Some(err),
            Error::PoolConn(ref err) => Some(err),
        }
    }
}

impl From<InitializationError> for Error {
    fn from(err: InitializationError) -> Error {
        Error::PoolInit(err)
    }
}

impl From<GetTimeout> for Error {
    fn from(err: GetTimeout) -> Error {
        Error::PoolConn(err)
    }
}

pub struct Storage<T: ManageConnection> {
    pool: Pool<T>,
}

impl<T: ManageConnection> Storage<T> {
    pub fn new(manager: T) -> Result<Self, Error> {
        let config = Config::default();
        let pool = Pool::new(config, manager)?;

        Ok(Storage { pool: pool })
    }

    pub fn get_conn(&self) -> Result<PooledConnection<T>, Error> {
        let conn = self.pool.get()?;

        Ok(conn)
    }
}

pub type Database = Storage<PostgresConnectionManager>;
pub type Cache = Storage<RedisConnectionManager>;
