use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Invitation {
    pub id: i64,
    pub user_id: i64,
    pub code: String,
    created_time: DateTime<Utc>,
    updated_time: Option<DateTime<Utc>>,
    removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    code: &str,
) -> Result<Invitation, ModelError> {
    let stmt = r#"
    INSERT INTO sso.invitations(user_id, code)
    VALUES ($1, $2)
    RETURNING *
    "#;
    let rows = pg_conn.query(stmt, &[&user_id, &code])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Invitation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            code: row.get("code"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

pub fn select<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
) -> Result<Vec<Invitation>, ModelError> {
    let stmt = r#"
    SELECT id,
           user_id,
           code,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.invitations
    WHERE user_id = $1
    "#;
    let rows = pg_conn.query(stmt, &[&user_id])?;

    let invitations = rows
        .iter()
        .map(|row| Invitation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            code: row.get("code"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<Invitation>>();

    Ok(invitations)
}

pub fn remove<T: GenericConnection>(
    pg_conn: &T,
    invitation_id: i64,
    user_id: i64,
) -> Result<Invitation, ModelError> {
    let stmt = r#"
    DELETE
    FROM sso.invitations
    WHERE id = $1 AND user_id = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&invitation_id, &user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Invitation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            code: row.get("code"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}
