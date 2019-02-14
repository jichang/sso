use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionType {
    NONE = 0,
    CREATE,
    SELECT,
    UPDATE,
    DELETE,
}

impl ActionType {
    fn from_i32(i: i32) -> ActionType {
        match i {
            1 => ActionType::CREATE,
            2 => ActionType::SELECT,
            3 => ActionType::UPDATE,
            4 => ActionType::DELETE,
            _ => ActionType::NONE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    NONE = 0,
    Application,
}

impl ResourceType {
    fn from_i32(i: i32) -> ResourceType {
        match i {
            1 => ResourceType::Application,
            _ => ResourceType::NONE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    resource_type: ResourceType,
    action_type: ActionType,
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
