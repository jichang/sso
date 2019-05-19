use super::quota;
use super::resource::{ActionType, ResourceType};
use super::role::RoleId;
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
    role_id: i32,
    user_id: i64,
    code: &str,
) -> Result<Invitation, ModelError> {
    let trans = pg_conn.transaction()?;

    let is_admin = role_id == RoleId::Admin as i32;
    let invitations_quota = quota::select(&trans, is_admin, user_id, ResourceType::Invitation)?;

    let select_stmt = r#"
    SELECT count(*) as used_quota
    FROM sso.invitations
    WHERE user_id = $1
    "#;

    let rows = pg_conn.query(&select_stmt, &[&user_id])?;
    if rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let row = rows.get(0);
    let used_quota: i64 = row.get("used_quota");
    if (used_quota >= invitations_quota.quota as i64) {
        return Err(ModelError::QuotaLimit);
    }

    let stmt = r#"
    INSERT INTO sso.invitations(user_id, code)
    VALUES ($1, $2)
    RETURNING *
    "#;
    let rows = trans.query(stmt, &[&user_id, &code])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        trans.set_commit();

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

pub fn update<T: GenericConnection>(
    pg_conn: &T,
    invitation_code: &str,
) -> Result<Invitation, ModelError> {
    let stmt = r#"
    UPDATE sso.invitations
    SET updated_time = now(), status = 1
    WHERE code = $1 and status = 0
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&invitation_code])?;
    if rows.len() != 1 {
        Err(ModelError::NotFound)
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
