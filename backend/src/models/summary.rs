use super::super::guards::permission::Permissions;
use super::quota::default_quota;
use super::resource::{ActionType, ResourceType};
use super::role::RoleId;
use super::Error as ModelError;
use postgres::GenericConnection;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryQuota {
    enabled: bool,
    total: i64,
    used: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    users: SummaryQuota,
    roles: SummaryQuota,
    groups: SummaryQuota,
    applications: SummaryQuota,
    authorizations: SummaryQuota,
    contacts: SummaryQuota,
    invitations: SummaryQuota,
}

pub fn select<T: GenericConnection>(
    pg_conn: &T,
    role_id: i32,
    user_id: i64,
    permissions: &Permissions,
) -> Result<Summary, ModelError> {
    let stmt = r#"
    SELECT 
        (SELECT count(*) FROM sso.users) as users_used,
        (SELECT count(*) FROM sso.roles) as roles_used,
        (SELECT count(*) FROM sso.groups) as groups_used,
        (SELECT count(*) FROM sso.applications WHERE user_id = $1) as applications_used,
        (SELECT count(*) FROM sso.authorizations WHERE user_id = $1) as authorizations_used,
        (SELECT count(*) FROM sso.contacts WHERE user_id = $1) as contacts_used,
        (SELECT count(*) FROM sso.invitations WHERE user_id = $1) as invitations_used
    "#;

    let rows = pg_conn.query(stmt, &[&user_id])?;
    if rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let row = rows.get(0);
    let users_enabled = permissions.contains(ResourceType::User, ActionType::CREATE);
    let users_used: i64 = row.get("users_used");
    let roles_enabled = permissions.contains(ResourceType::Role, ActionType::CREATE);
    let roles_used: i64 = row.get("roles_used");
    let groups_enabled = permissions.contains(ResourceType::Group, ActionType::CREATE);
    let groups_used: i64 = row.get("groups_used");
    let applications_enabled = permissions.contains(ResourceType::Application, ActionType::CREATE);
    let applications_used: i64 = row.get("applications_used");
    let authorizations_enabled =
        permissions.contains(ResourceType::Authorization, ActionType::CREATE);
    let authorizations_used: i64 = row.get("authorizations_used");
    let contacts_enabled = permissions.contains(ResourceType::Contact, ActionType::CREATE);
    let contacts_used: i64 = row.get("contacts_used");
    let invitations_enabled = permissions.contains(ResourceType::Invitation, ActionType::CREATE);
    let invitations_used: i64 = row.get("invitations_used");

    let is_admin = role_id == RoleId::Admin as i32;

    Ok(Summary {
        users: SummaryQuota {
            enabled: users_enabled,
            total: default_quota(is_admin, ResourceType::User),
            used: if users_enabled { users_used } else { 0 },
        },
        roles: SummaryQuota {
            enabled: roles_enabled,
            total: default_quota(is_admin, ResourceType::Role),
            used: if roles_enabled { roles_used } else { 0 },
        },
        groups: SummaryQuota {
            enabled: groups_enabled,
            total: default_quota(is_admin, ResourceType::Group),
            used: if groups_enabled { groups_used } else { 0 },
        },
        applications: SummaryQuota {
            enabled: applications_enabled,
            total: default_quota(is_admin, ResourceType::Application),
            used: applications_used,
        },
        invitations: SummaryQuota {
            enabled: invitations_enabled,
            total: default_quota(is_admin, ResourceType::Invitation),
            used: invitations_used,
        },
        authorizations: SummaryQuota {
            enabled: true,
            total: default_quota(is_admin, ResourceType::Authorization),
            used: authorizations_used,
        },
        contacts: SummaryQuota {
            enabled: true,
            total: default_quota(is_admin, ResourceType::Contact),
            used: contacts_used,
        },
    })
}
