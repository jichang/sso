use super::super::guards::permission::Permissions;
use super::super::models::permission::{ActionType, ResourceType};
use super::Error as ModelError;
use postgres::GenericConnection;

const QUOTA_LIMIT: i64 = 10;

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
    let authorizations_used: i64 = row.get("authorizations_used");
    let contacts_used: i64 = row.get("contacts_used");
    let invitations_used: i64 = row.get("invitations_used");

    Ok(Summary {
        users: SummaryQuota {
            enabled: users_enabled,
            total: i64::max_value(),
            used: if users_enabled { users_used } else { 0 },
        },
        roles: SummaryQuota {
            enabled: roles_enabled,
            total: i64::max_value(),
            used: if roles_enabled { roles_used } else { 0 },
        },
        groups: SummaryQuota {
            enabled: groups_enabled,
            total: i64::max_value(),
            used: if groups_enabled { groups_used } else { 0 },
        },
        applications: SummaryQuota {
            enabled: applications_enabled,
            total: QUOTA_LIMIT,
            used: if applications_enabled {
                applications_used
            } else {
                0
            },
        },
        authorizations: SummaryQuota {
            enabled: true,
            total: QUOTA_LIMIT,
            used: authorizations_used,
        },
        contacts: SummaryQuota {
            enabled: true,
            total: QUOTA_LIMIT,
            used: contacts_used,
        },
        invitations: SummaryQuota {
            enabled: true,
            total: QUOTA_LIMIT,
            used: invitations_used,
        },
    })
}
