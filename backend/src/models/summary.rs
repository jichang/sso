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
    roles: SummaryQuota,
    groups: SummaryQuota,
    applications: SummaryQuota,
    authorizations: SummaryQuota,
    contacts: SummaryQuota,
}

pub fn select<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    permissions: &Permissions,
) -> Result<Summary, ModelError> {
    let stmt = r#"
    SELECT 
        (SELECT count(*) FROM sso.roles) as roles_used,
        (SELECT count(*) FROM sso.groups) as groups_used,
        (SELECT count(*) FROM sso.applications WHERE user_id = $1) as applications_used,
        (SELECT count(*) FROM sso.authorizations WHERE user_id = $1) as authorizations_used,
        (SELECT count(*) FROM sso.contacts WHERE user_id = $1) as contacts_used
    "#;

    let rows = pg_conn.query(stmt, &[&user_id])?;
    if rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let row = rows.get(0);
    let roles_enabled = permissions.contains(ResourceType::Role, ActionType::CREATE);
    let roles_used: i64 = row.get("roles_used");
    let groups_enabled = permissions.contains(ResourceType::Group, ActionType::CREATE);
    let groups_used: i64 = row.get("groups_used");
    let applications_enabled = permissions.contains(ResourceType::Application, ActionType::CREATE);
    let applications_used: i64 = row.get("applications_used");
    let authorizations_used: i64 = row.get("authorizations_used");
    let contacts_used: i64 = row.get("contacts_used");

    Ok(Summary {
        roles: SummaryQuota {
            enabled: roles_enabled,
            total: QUOTA_LIMIT,
            used: if roles_enabled { roles_used } else { 0 },
        },
        groups: SummaryQuota {
            enabled: groups_enabled,
            total: QUOTA_LIMIT,
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
    })
}
