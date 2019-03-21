use super::crypto;
use super::crypto::Plaintext;
use super::role::Role;
use super::username::Username;
use super::Error as ModelError;
use super::PaginatorParams;
use super::ResourceCollection;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub union_id: Uuid,
    pub role: Role,
    pub account: Account,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserSource {
    Website = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Normal = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountVerify {}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    union_id: Uuid,
    username: &str,
    password: &str,
    group_id: i64,
) -> Result<User, ModelError> {
    let username = Username::new(username)?;
    let plaintext = Plaintext::new(password)?;
    let ciphertext = crypto::generate(&plaintext)?;

    let trans = pg_conn.transaction()?;

    let stmt = r#"
        INSERT INTO sso.users(union_id)
        VALUES ($1)
        RETURNING *
    "#;
    let user_rows = trans.query(&stmt, &[&union_id])?;
    if user_rows.len() != 1 {
        return Err(ModelError::Unknown);
    }
    let user_row = user_rows.get(0);
    let user_id: i64 = user_row.get("id");

    let stmt = r#"
        INSERT INTO sso.accounts(user_id, username, salt, hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#;
    let account_rows = trans.query(
        &stmt,
        &[&user_id, &username, &ciphertext.salt, &ciphertext.hash],
    )?;
    if account_rows.len() != 1 {
        return Err(ModelError::Unknown);
    }
    let account_row = account_rows.get(0);

    let stmt = r#"
        INSERT INTO sso.group_users(group_id, user_id)
        VALUES ($1, $2)
        RETURNING *
    "#;
    let group_rows = trans.query(&stmt, &[&group_id, &user_id])?;
    if group_rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let stmt = r#"
        SELECT *
        FROM sso.roles as roles
        JOIN sso.group_roles as group_roles
            ON group_roles.role_id = roles.id
        WHERE group_roles.group_id = $1;
    "#;
    let role_rows = trans.query(&stmt, &[&group_id])?;
    if role_rows.len() != 1 {
        return Err(ModelError::Unknown);
    }
    let role_row = role_rows.get(0);

    trans.set_commit();

    Ok(User {
        id: user_row.get("id"),
        union_id: user_row.get("union_id"),
        role: Role {
            id: role_row.get("id"),
            name: role_row.get("name"),
            permissions: vec![],
            created_time: role_row.get("created_time"),
            updated_time: role_row.get("updated_time"),
            removed_time: role_row.get("removed_time"),
            status: role_row.get("status"),
        },
        account: Account {
            id: account_row.get("id"),
            username: account_row.get("username"),
            created_time: account_row.get("created_time"),
            updated_time: account_row.get("updated_time"),
            removed_time: account_row.get("removed_time"),
            status: role_row.get("status"),
        },
        created_time: role_row.get("created_time"),
        updated_time: role_row.get("updated_time"),
        removed_time: role_row.get("removed_time"),
        status: user_row.get("status"),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    NotMatch,
}

pub fn auth<T: GenericConnection>(
    pg_conn: &T,
    username: &str,
    password: &str,
) -> Result<User, ModelError> {
    let plaintext = Plaintext::new(password)?;

    let trans = pg_conn.transaction()?;

    let stmt = r#"
    SELECT users.id as user_id,
           users.union_id as user_union_id,
           users.created_time as user_created_time,
           users.updated_time as user_updated_time,
           users.removed_time as user_removed_time,
           users.status as user_status,
           accounts.id as account_id,
           accounts.username as account_username,
           accounts.salt as account_salt,
           accounts.hash as account_hash,
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
    FROM sso.accounts as accounts
    LEFT JOIN sso.users as users ON users.id = accounts.user_id
    LEFT JOIN sso.group_users as group_users ON group_users.user_id = users.id
    LEFT JOIN sso.group_roles as group_roles ON group_roles.group_id = group_users.group_id
    LEFT JOIN sso.roles as roles ON roles.id = group_roles.role_id
    WHERE username = $1;
    "#;

    let rows = trans.query(&stmt, &[&username])?;
    if rows.len() == 0 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);
        let hash: Vec<u8> = row.get("account_hash");
        let salt: String = row.get("account_salt");

        match crypto::compare(&plaintext, salt, hash) {
            Ok(_) => {
                trans.set_commit();

                Ok(User {
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
            }
            Err(err) => Err(ModelError::InvalidParam(
                "password".to_string(),
                Box::new(err),
            )),
        }
    }
}

pub fn select_user<T: GenericConnection>(pg_conn: &T, user_id: i64) -> Result<User, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    SELECT users.id as user_id,
           users.union_id as user_union_id,
           users.created_time as user_created_time,
           users.updated_time as user_updated_time,
           users.removed_time as user_removed_time,
           users.status as user_status,
           accounts.id as account_id,
           accounts.username as account_username,
           accounts.salt as account_salt,
           accounts.hash as account_hash,
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
    FROM sso.accounts as accounts
    LEFT JOIN sso.users as users ON users.id = accounts.user_id
    LEFT JOIN sso.group_users as group_users ON group_users.user_id = users.id
    LEFT JOIN sso.group_roles as group_roles ON group_roles.group_id = group_users.group_id
    LEFT JOIN sso.roles as roles ON roles.id = group_roles.role_id
    WHERE users.id = $1;
    "#;

    let rows = trans.query(&stmt, &[&user_id])?;
    if rows.len() == 0 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);

        Ok(User {
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
    }
}

pub fn change_password<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    old_password: &str,
    new_password: &str,
) -> Result<(), ModelError> {
    let plaintext = Plaintext::new(old_password)?;

    let trans = pg_conn.transaction()?;

    let stmt = r#"
    SELECT id, salt, hash
    FROM sso.accounts
    WHERE user_id = $1;
    "#;

    let rows = trans.query(&stmt, &[&user_id])?;
    if rows.len() == 0 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);
        let account_id: i64 = row.get("id");
        let hash: Vec<u8> = row.get("hash");
        let salt: String = row.get("salt");

        match crypto::compare(&plaintext, salt, hash) {
            Ok(_) => {
                let plaintext = Plaintext::new(new_password)?;
                let ciphertext = crypto::generate(&plaintext)?;

                let stmt = r#"
                    UPDATE sso.accounts
                    SET salt = $1, hash = $2
                    WHERE user_id = $3
                    RETURNING id
                "#;
                let rows = trans.query(&stmt, &[&ciphertext.salt, &ciphertext.hash, &user_id])?;
                if rows.len() == 0 {
                    Err(ModelError::NotFound)
                } else {
                    trans.set_commit();

                    Ok(())
                }
            }
            Err(err) => Err(ModelError::InvalidParam(
                "old_password".to_string(),
                Box::new(err),
            )),
        }
    }
}

pub fn select_users<T: GenericConnection>(
    pg_conn: &T,
    group_id: Option<i64>,
    params: &PaginatorParams,
) -> Result<ResourceCollection<User>, ModelError> {
    let where_clause = if group_id.is_some() {
        "WHERE group_users.group_id = $3"
    } else {
        ""
    };

    let stmt = format!(
        "
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
    {}
    LIMIT $1
    OFFSET $2
    ",
        where_clause
    );

    let rows = match group_id {
        Some(group_id) => pg_conn.query(&stmt, &[&params.limit, &params.offset, &group_id])?,
        None => pg_conn.query(&stmt, &[&params.limit, &params.offset])?,
    };

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
