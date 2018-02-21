use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use uuid::Uuid;
use super::Error as ModelError;
use super::role::Role;
use super::username::Username;
use super::crypto;
use super::crypto::Plaintext;

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

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    union_id: Uuid,
    username: &str,
    password: &str,
    role_id: i32,
) -> Result<User, ModelError> {
    let username = Username::new(username)?;
    let plaintext = Plaintext::new(password)?;
    let ciphertext = crypto::generate(&plaintext)?;

    let trans = pg_conn.transaction()?;

    let stmt = r#"
    INSERT INTO sso.users(role_id, union_id)
    VALUES ($1, $2)
    RETURNING *
    "#;
    let rows = trans.query(&stmt, &[&role_id, &union_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let user_row = rows.get(0);
        let user_id: i64 = user_row.get("id");

        let stmt = r#"
        INSERT INTO sso.accounts(user_id, username, salt, hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#;
        let rows = trans.query(
            &stmt,
            &[&user_id, &username, &ciphertext.salt, &ciphertext.hash],
        )?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let account_row = rows.get(0);

            let stmt = r#"
            SELECT *
            FROM sso.roles
            WHERE id = $1;
            "#;
            let rows = trans.query(&stmt, &[&role_id])?;
            if rows.len() != 1 {
                Err(ModelError::Unknown)
            } else {
                let role_row = rows.get(0);

                trans.set_commit();

                Ok(User {
                    id: user_row.get("id"),
                    union_id: user_row.get("union_id"),
                    role: Role {
                        id: role_row.get("id"),
                        name: role_row.get("name"),
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
        }
    }
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
    LEFT JOIN sso.roles as roles ON users.role_id = roles.id
    WHERE username = $1;
    "#;

    let rows = pg_conn.query(&stmt, &[&username])?;
    if rows.len() == 0 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);
        let hash: Vec<u8> = row.get("account_hash");
        let salt: String = row.get("account_salt");

        let _ = crypto::compare(&plaintext, salt, hash)?;

        Ok(User {
            id: row.get("user_id"),
            union_id: row.get("user_union_id"),
            role: Role {
                id: row.get("role_id"),
                name: row.get("role_name"),
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
