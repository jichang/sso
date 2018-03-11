use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use super::Error as ModelError;

pub enum ActionId {
    UsersSignup = 1,
    UsersAccountsCreate,
    UsersAccountsVerify,
    UsersContactsCreate,
    UsersContactsRemove,
    UsersApplicationsCreate,
    UsersApplicationsUpdate,
    UsersApplicationsRemove,
    UsersAuthorizationsCreate,
    UsersAuthorizationsUpdate,
    UsersAuthorizationsRemove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn select<T: GenericConnection>(pg_conn: &T) -> Result<Vec<Action>, ModelError> {
    let stmt = r#"
    SELECT id, key, name, description, created_time, updated_time, removed_time, status
    FROM sso.actions
    "#;
    let rows = pg_conn.query(stmt, &[])?;

    let actions = rows.iter()
        .map(|row| Action {
            id: row.get("id"),
            key: row.get("key"),
            name: row.get("name"),
            description: row.get("description"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<Action>>();

    Ok(actions)
}

#[cfg(test)]
mod test {
    use postgres::{Connection, TlsMode};
    use super::ActionId;

    #[test]
    pub fn test_actions_select() {
        let conn =
            Connection::connect("postgres://feblr:feblr@localhost/feblr", TlsMode::None).unwrap();
        let actions = super::select(&conn).unwrap();
        assert_eq!(actions.len(), 11);

        actions
            .into_iter()
            .map(|action| match &action {
                _ if action.id == ActionId::UsersSignup as i32 => {
                    assert_eq!(action.key, "users.signup")
                }
                _ if action.id == ActionId::UsersAccountsCreate as i32 => {
                    assert_eq!(action.key, "users.accounts.create")
                }
                _ if action.id == ActionId::UsersAccountsVerify as i32 => {
                    assert_eq!(action.key, "users.accounts.verify")
                }
                _ if action.id == ActionId::UsersContactsCreate as i32 => {
                    assert_eq!(action.key, "users.contacts.create")
                }
                _ if action.id == ActionId::UsersContactsRemove as i32 => {
                    assert_eq!(action.key, "users.contacts.remove")
                }
                _ if action.id == ActionId::UsersApplicationsCreate as i32 => {
                    assert_eq!(action.key, "users.applications.create")
                }
                _ if action.id == ActionId::UsersApplicationsUpdate as i32 => {
                    assert_eq!(action.key, "users.applications.update")
                }
                _ if action.id == ActionId::UsersApplicationsRemove as i32 => {
                    assert_eq!(action.key, "users.applications.remove")
                }
                _ if action.id == ActionId::UsersAuthorizationsCreate as i32 => {
                    assert_eq!(action.key, "users.authorizations.create")
                }
                _ if action.id == ActionId::UsersAuthorizationsUpdate as i32 => {
                    assert_eq!(action.key, "users.authorizations.update")
                }
                _ if action.id == ActionId::UsersAuthorizationsRemove as i32 => {
                    assert_eq!(action.key, "users.authorizations.remove")
                }
                _ => panic!("unknown action"),
            })
            .collect::<()>();
    }
}
