use super::resource::{ActionType, ResourceType};
use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

pub const DEFAULT_QUOTA_LIMIT_FOR_NORMAL_USER: i64 = 3;
pub const DEFAULT_QUOTA_LIMIT_FOR_ADMIN_USER: i64 = i64::max_value();

#[derive(Debug, Serialize, Deserialize)]
pub struct Quota {
    pub resource_type: ResourceType,
    pub quota: i64,
    pub status: i32,
}

pub fn default_quota(is_admin: bool, resource_type: ResourceType) -> i64 {
    match (is_admin, resource_type) {
        (true, _) => DEFAULT_QUOTA_LIMIT_FOR_ADMIN_USER,
        (false, ResourceType::Authorization) => i64::max_value(),
        (false, ResourceType::Contact) => i64::max_value(),
        (false, _) => 0,
    }
}

pub fn create<C: GenericConnection>(
    pg_conn: &C,
    user_id: i64,
    quota: &Quota,
) -> Result<Quota, ModelError> {
    let stmt = r#"
    INSERT INTO sso.user_quotas(user_id, resource_type, quota)
    VALUES($1, $2, $3)
    RETURNING *
    "#;

    let rows = pg_conn.query(
        &stmt,
        &[&user_id, &(quota.resource_type as i32), &quota.quota],
    )?;

    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let quota = Quota {
            resource_type: ResourceType::from_i32(row.get("resource_type")),
            quota: row.get("quota"),
            status: row.get("status"),
        };

        Ok(quota)
    }
}

pub fn select<C: GenericConnection>(
    pg_conn: &C,
    is_admin: bool,
    user_id: i64,
    resource_type: ResourceType,
) -> Result<Quota, ModelError> {
    let stmt = r#"
    SELECT resource_type, quota
    FROM sso.user_quotas
    WHERE user_id = $1 AND resource_type = $2
    "#;
    let rows = pg_conn.query(&stmt, &[&user_id, &(resource_type as i32)])?;

    let quota = rows
        .iter()
        .map(|row| {
            let quota: i32 = row.get("quota");

            return quota as i64;
        })
        .fold(
            default_quota(is_admin, resource_type),
            |total_quota, quota| total_quota + quota,
        );

    return Ok(Quota {
        resource_type: resource_type,
        quota: quota,
        status: 0,
    });
}
