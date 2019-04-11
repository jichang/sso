use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;
use std::net::IpAddr;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ActionType {
    NONE = 0,
    CREATE = 1,
    SELECT = 2,
    UPDATE = 3,
    DELETE = 4,
}

impl ActionType {
    pub fn from_i32(i: i32) -> ActionType {
        match i {
            1 => ActionType::CREATE,
            2 => ActionType::SELECT,
            3 => ActionType::UPDATE,
            4 => ActionType::DELETE,
            _ => ActionType::NONE,
        }
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ResourceType {
    NONE = 0,
    Application = 1,
    Group = 2,
    GroupUser = 3,
    Role = 4,
    RoleUser = 5,
    RolePermission = 6,
    User = 7,
    Contact = 8,
    Invitation = 9,
}

impl ResourceType {
    pub fn from_i32(i: i32) -> ResourceType {
        match i {
            1 => ResourceType::Application,
            2 => ResourceType::Group,
            3 => ResourceType::GroupUser,
            4 => ResourceType::Role,
            5 => ResourceType::RoleUser,
            6 => ResourceType::RolePermission,
            7 => ResourceType::User,
            8 => ResourceType::Contact,
            9 => ResourceType::Invitation,
            _ => ResourceType::NONE,
        }
    }
}

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
