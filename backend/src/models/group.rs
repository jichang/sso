use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;

pub enum GroupId {
    Admin = 1,
    Normal = 2,
    Guest = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn select<T: GenericConnection>(pg_conn: &T) -> Result<Vec<Group>, ModelError> {
    let stmt = r#"
    SELECT id, name, created_time, updated_time, removed_time, status
    FROM sso.groups
    "#;
    let rows = pg_conn.query(stmt, &[])?;

    let groups = rows
        .iter()
        .map(|row| Group {
            id: row.get("id"),
            name: row.get("name"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<Group>>();

    Ok(groups)
}

#[cfg(test)]
mod test {
    use super::GroupId;
    use postgres::{Connection, TlsMode};

    #[test]
    pub fn test_groups_select() {
        let conn =
            Connection::connect("postgres://feblr:feblr@localhost/feblr", TlsMode::None).unwrap();
        let groups = super::select(&conn).unwrap();
        assert_eq!(groups.len(), 3);

        groups.into_iter().for_each(|group| match &group {
            _ if group.id == GroupId::Admin as i64 => assert_eq!(group.name, "admin"),
            _ if group.id == GroupId::Normal as i64 => assert_eq!(group.name, "normal"),
            _ if group.id == GroupId::Guest as i64 => assert_eq!(group.name, "guest"),
            _ => panic!("unknown group"),
        });
    }
}
