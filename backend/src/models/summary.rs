use super::Error as ModelError;
use postgres::GenericConnection;

const QUOTA_LIMIT: i64 = 10;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryQuota {
    total: i64,
    used: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    applications: SummaryQuota,
    authorizations: SummaryQuota,
    contacts: SummaryQuota,
}

pub fn select<T: GenericConnection>(pg_conn: &T, user_id: i64) -> Result<Summary, ModelError> {
    let stmt = r#"
    SELECT 
        (SELECT count(*) FROM sso.applications WHERE user_id = $1) as applications_used,
        (SELECT count(*) FROM sso.authorizations WHERE user_id = $1) as authorizations_used,
        (SELECT count(*) FROM sso.contacts WHERE user_id = $1) as contacts_used
    "#;

    let rows = pg_conn.query(stmt, &[&user_id])?;
    if rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let row = rows.get(0);
    let applications_used: i64 = row.get("applications_used");
    let authorizations_used: i64 = row.get("authorizations_used");
    let contacts_used: i64 = row.get("contacts_used");

    Ok(Summary {
        applications: SummaryQuota {
            total: QUOTA_LIMIT,
            used: applications_used,
        },
        authorizations: SummaryQuota {
            total: QUOTA_LIMIT,
            used: authorizations_used,
        },
        contacts: SummaryQuota {
            total: QUOTA_LIMIT,
            used: contacts_used,
        },
    })
}
