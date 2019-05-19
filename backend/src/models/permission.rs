use super::resource::{ActionType, ResourceType};
use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Permission {
    pub resource_type: ResourceType,
    pub action_type: ActionType,
}

pub fn select<C: GenericConnection>(
    pg_conn: &C,
    role_id: i32,
) -> Result<Vec<Permission>, ModelError> {
    let stmt = r#"
    SELECT resource_type, action_type
    FROM sso.role_permissions as role_permissions
    LEFT JOIN sso.permissions as permissions ON role_permissions.permission_id = permissions.id
    WHERE role_id = $1
    "#;
    let rows = pg_conn.query(&stmt, &[&role_id])?;

    let permissions = rows
        .iter()
        .map(|row| {
            let action_type: i32 = row.get("action_type");
            let resource_type: i32 = row.get("resource_type");

            Permission {
                resource_type: ResourceType::from_i32(resource_type),
                action_type: ActionType::from_i32(action_type),
            }
        })
        .collect::<Vec<Permission>>();

    return Ok(permissions);
}

pub fn select_all<C: GenericConnection>(pg_conn: &C) -> Result<Vec<Permission>, ModelError> {
    let stmt = r#"
    SELECT resource_type, action_type
    FROM sso.permissions
    "#;
    let rows = pg_conn.query(&stmt, &[])?;

    let permissions = rows
        .iter()
        .map(|row| {
            let action_type: i32 = row.get("action_type");
            let resource_type: i32 = row.get("resource_type");

            Permission {
                resource_type: ResourceType::from_i32(resource_type),
                action_type: ActionType::from_i32(action_type),
            }
        })
        .collect::<Vec<Permission>>();

    return Ok(permissions);
}
