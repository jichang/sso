use super::application::{Application, Scope};
use super::Error as ModelError;
use chrono::{DateTime, Utc};
use hex;
use oath::{totp_raw_now, HashType};
use postgres::GenericConnection;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Totp {
    pub id: i64,
    pub user_id: i64,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    secret: &Vec<u8>,
) -> Result<Totp, ModelError> {
    let stmt = r#"
    INSERT INTO sso.totp(user_id, secret)
    VALUES ($1, $2)
    ON CONFLICT ON CONSTRAINT totp_unique_key DO UPDATE SET updated_time = now(), secret = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&user_id, &secret])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Totp {
            id: row.get("id"),
            user_id: row.get("user_id"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

pub fn verify<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    code: u64,
) -> Result<bool, ModelError> {
    let stmt = r#"
    SELECT user_id, secret
    FROM sso.totp
    WHERE user_id = $1
    "#;

    let rows = pg_conn.query(&stmt, &[&user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let secret: Vec<u8> = row.get("secret");

        Ok(code == totp_raw_now(&secret, 6, 0, 30, &HashType::SHA1))
    }
}
