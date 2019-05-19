use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

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
    Authorization = 10,
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
