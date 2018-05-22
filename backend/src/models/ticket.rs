use super::Error as ModelError;
use hex;
use postgres::GenericConnection;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i64,
    pub open_id: Option<Uuid>,
    pub access_token: String,
    pub refresh_token: String,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    authorization_id: i64,
    open_id: &Uuid,
    access_token: &Vec<u8>,
    refresh_token: &Vec<u8>,
) -> Result<Ticket, ModelError> {
    let stmt = r#"
    INSERT INTO sso.tickets(authorization_id, access_token, refresh_token)
    VALUES ($1, $2, $3)
    ON CONFLICT ON CONSTRAINT tickets_unique_key
    DO UPDATE SET updated_time = now(), access_token = $2, refresh_token = $3
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&authorization_id, &access_token, &refresh_token])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Ticket {
            id: row.get("id"),
            open_id: Some(open_id.clone()),
            access_token: hex::encode(access_token),
            refresh_token: hex::encode(refresh_token),
        })
    }
}

pub fn update<T: GenericConnection>(
    pg_conn: &T,
    client_id: &str,
    access_token: &Vec<u8>,
    refresh_token: &Vec<u8>,
) -> Result<Ticket, ModelError> {
    let stmt = r#"
    UPDATE sso.tickets
    SET access_token = $1
    WHERE refresh_token = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&access_token, &refresh_token])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Ticket {
            id: row.get("id"),
            open_id: None,
            access_token: hex::encode(access_token),
            refresh_token: hex::encode(refresh_token),
        })
    }
}
