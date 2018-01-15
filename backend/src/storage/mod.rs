use r2d2::{Error, Pool, PooledConnection, ManageConnection};
use r2d2_postgres::PostgresConnectionManager;
use r2d2_redis::RedisConnectionManager;

pub struct Storage<T: ManageConnection> {
    pool: Pool<T>,
}

impl<T: ManageConnection> Storage<T> {
    pub fn new(manager: T) -> Result<Self, Error> {
        let pool = Pool::new(manager)?;

        Ok(Storage { pool: pool })
    }

    pub fn get_conn(&self) -> Result<PooledConnection<T>, Error> {
        let conn = self.pool.get()?;

        Ok(conn)
    }
}

pub type Database = Storage<PostgresConnectionManager>;
pub type Cache = Storage<RedisConnectionManager>;
