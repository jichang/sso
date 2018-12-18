use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;

pub enum RoleId {
    Admin = 1,
    Normal = 2,
    Guest = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn select<T: GenericConnection>(pg_conn: &T) -> Result<Vec<Role>, ModelError> {
    let stmt = r#"
    SELECT id, name, created_time, updated_time, removed_time, status
    FROM sso.roles
    "#;
    let rows = pg_conn.query(stmt, &[])?;

    let roles = rows
        .iter()
        .map(|row| Role {
            id: row.get("id"),
            name: row.get("name"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<Role>>();

    Ok(roles)
}

#[cfg(test)]
mod test {
    use super::RoleId;
    use postgres::{Connection, TlsMode};

    #[test]
    pub fn test_roles_select() {
        let conn =
            Connection::connect("postgres://feblr:feblr@localhost/feblr", TlsMode::None).unwrap();
        let roles = super::select(&conn).unwrap();
        assert_eq!(roles.len(), 3);

        roles.into_iter().for_each(|role| match &role {
            _ if role.id == RoleId::Admin as i32 => assert_eq!(role.name, "admin"),
            _ if role.id == RoleId::Normal as i32 => assert_eq!(role.name, "normal"),
            _ if role.id == RoleId::Guest as i32 => assert_eq!(role.name, "guest"),
            _ => panic!("unknown role"),
        });
    }
}
