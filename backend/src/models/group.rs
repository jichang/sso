use super::role::Role;
use super::user::{Account, User};
use super::Error as ModelError;
use super::PaginatorParams;
use super::ResourceCollection;
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

pub fn select_users<T: GenericConnection>(
    pg_conn: &T,
    group_id: i64,
    params: &PaginatorParams,
) -> Result<ResourceCollection<User>, ModelError> {
    let stmt = r#"
    SELECT count(*) OVER() AS total,
           users.id as user_id,
           users.union_id as user_union_id,
           users.created_time as user_created_time,
           users.updated_time as user_updated_time,
           users.removed_time as user_removed_time,
           users.status as user_status,
           accounts.id as account_id,
           accounts.username as account_username,
           accounts.created_time as account_created_time,
           accounts.updated_time as account_updated_time,
           accounts.removed_time as account_removed_time,
           accounts.status as account_status,
           roles.id as role_id,
           roles.name as role_name,
           roles.created_time as role_created_time,
           roles.updated_time as role_updated_time,
           roles.removed_time as role_removed_time,
           roles.status as role_status
    FROM sso.group_users as group_users
    LEFT JOIN sso.users as users ON users.id = group_users.user_id
    LEFT JOIN sso.accounts as accounts ON accounts.user_id = group_users.user_id
    LEFT JOIN sso.group_roles as group_roles ON group_roles.group_id = group_users.group_id
    LEFT JOIN sso.roles as roles ON roles.id = group_roles.role_id
    WHERE group_users.group_id = $1
    LIMIT $2
    OFFSET $3
    "#;
    let rows = pg_conn.query(stmt, &[&group_id, &params.limit, &params.offset])?;

    let total: i64 = match rows.len() {
        0 => 0,
        _ => {
            let row = rows.get(0);
            row.get("total")
        }
    };

    let users = rows
        .iter()
        .map(|row| User {
            id: row.get("user_id"),
            union_id: row.get("user_union_id"),
            role: Role {
                id: row.get("role_id"),
                name: row.get("role_name"),
                permissions: vec![],
                created_time: row.get("role_created_time"),
                updated_time: row.get("role_updated_time"),
                removed_time: row.get("role_removed_time"),
                status: row.get("role_status"),
            },
            account: Account {
                id: row.get("account_id"),
                username: row.get("account_username"),
                created_time: row.get("account_created_time"),
                updated_time: row.get("account_updated_time"),
                removed_time: row.get("account_removed_time"),
                status: row.get("account_status"),
            },
            created_time: row.get("user_created_time"),
            updated_time: row.get("user_updated_time"),
            removed_time: row.get("user_removed_time"),
            status: row.get("user_status"),
        })
        .collect::<Vec<User>>();

    Ok(ResourceCollection {
        total: total,
        items: users,
    })
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
