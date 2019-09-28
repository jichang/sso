use super::quota;
use super::resource::{ActionType, ResourceType};
use super::role::RoleId;
use super::Error as ModelError;
use chrono::{DateTime, Utc};
use hex;
use postgres::GenericConnection;
use std::convert::From;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub website_uri: String,
    pub callback_uri: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    role_id: i32,
    user_id: i64,
    name: &str,
    website_uri: &str,
    callback_uri: &str,
) -> Result<Application, ModelError> {
    let website_uri = Url::parse(website_uri)
        .map_err(|err| ModelError::InvalidParam(String::from("website_uri"), Box::new(err)))?;
    let callback_uri = Url::parse(callback_uri)
        .map_err(|err| ModelError::InvalidParam(String::from("callback_uri"), Box::new(err)))?;

    let trans = pg_conn.transaction()?;

    let is_admin = role_id == RoleId::Admin as i32;
    let applications_quota = quota::select(&trans, is_admin, user_id, ResourceType::Application)?;

    let select_stmt = r#"
    SELECT count(*) as used_quota
    FROM sso.applications
    WHERE user_id = $1
    "#;

    let rows = pg_conn.query(&select_stmt, &[&user_id])?;
    if rows.len() != 1 {
        return Err(ModelError::Unknown);
    }

    let row = rows.get(0);
    let used_quota: i64 = row.get("used_quota");
    if (used_quota >= applications_quota.quota as i64) {
        return Err(ModelError::QuotaLimit);
    }

    let stmt = r#"
    INSERT INTO sso.applications(user_id, name, website_uri, callback_uri)
    VALUES ($1, $2, $3, $4)
    RETURNING *
    "#;

    let rows = trans.query(
        &stmt,
        &[
            &user_id,
            &name,
            &website_uri.as_str(),
            &callback_uri.as_str(),
        ],
    )?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        trans.set_commit();

        let row = rows.get(0);

        Ok(Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            callback_uri: row.get("callback_uri"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

pub fn select<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
) -> Result<Vec<Application>, ModelError> {
    let stmt = r#"
    SELECT id,
           user_id,
           name,
           website_uri,
           callback_uri,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.applications
    WHERE user_id = $1
    "#;
    let rows = pg_conn
        .query(stmt, &[&user_id])
        .map_err(|err| ModelError::Database(err))?;

    let mut applications = vec![];
    for row in &rows {
        let application = Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            callback_uri: row.get("callback_uri"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        };

        applications.push(application);
    }

    Ok(applications)
}

pub fn select_one<T: GenericConnection>(
    pg_conn: &T,
    client_id: &Vec<u8>,
) -> Result<Application, ModelError> {
    let stmt = r#"
    SELECT applications.id,
           applications.user_id,
           applications.name,
           applications.website_uri,
           applications.callback_uri,
           applications.created_time,
           applications.updated_time,
           applications.removed_time,
           applications.status
    FROM sso.applications as applications
    LEFT JOIN sso.application_secrets as secrets ON secrets.application_id = applications.id
    WHERE secrets.client_id = $1
    "#;
    let rows = pg_conn
        .query(stmt, &[&client_id])
        .map_err(|err| ModelError::Database(err))?;

    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        let application = Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            callback_uri: row.get("callback_uri"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        };
        Ok(application)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchConditions {
    pub open_id: Uuid,
    pub access_token: Vec<u8>,
}

pub fn select_many<T: GenericConnection>(
    pg_conn: &T,
    conditions: &SearchConditions,
) -> Result<Vec<Application>, ModelError> {
    let stmt = r#"
    SELECT authorizations.id AS id,
           authorizations.user_id AS user_id,
           authorizations.created_time AS created_time,
           authorizations.updated_time AS updated_time,
           authorizations.removed_time AS removed_time,
           authorizations.status
    FROM sso.authorizations AS authorizations
    LEFT JOIN sso.tickets AS tickets ON tickets.authorization_id = authorizations.id
    LEFT JOIN sso.scopes AS scopes ON scopes.id = authorizations.scope_id
    WHERE authorizations.open_id = $1 AND tickets.access_token = $2 AND scopes.name = 'user.apps'
    "#;
    let rows = pg_conn
        .query(stmt, &[&conditions.open_id, &conditions.access_token])
        .map_err(|err| ModelError::Database(err))?;

    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let user_id: i64 = row.get("user_id");
        select(pg_conn, user_id)
    }
}

pub fn remove<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    application_id: i64,
) -> Result<Application, ModelError> {
    let stmt = r#"
    DELETE
    FROM sso.applications
    WHERE id = $1 AND user_id = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&application_id, &user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            callback_uri: row.get("callback_uri"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub id: i64,
    pub application_id: i64,
    pub client_id: String,
    pub client_secret: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create_secret<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
    client_id: &Vec<u8>,
    client_secret: &Vec<u8>,
) -> Result<Secret, ModelError> {
    let stmt = r#"
    INSERT INTO sso.application_secrets(application_id, client_id, client_secret)
    VALUES ($1, $2, $3)
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&application_id, &client_id, &client_secret])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Secret {
            id: row.get("id"),
            application_id: row.get("application_id"),
            client_id: hex::encode(client_id),
            client_secret: hex::encode(client_secret),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

pub fn select_secrets<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
) -> Result<Vec<Secret>, ModelError> {
    let stmt = r#"
    SELECT id,
           application_id,
           client_id,
           client_secret,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.application_secrets
    WHERE application_id = $1
    "#;
    let rows = pg_conn
        .query(stmt, &[&application_id])
        .map_err(|err| ModelError::Database(err))?;

    let mut secrets = vec![];
    for row in &rows {
        let client_id: Vec<u8> = row.get("client_id");
        let client_secret: Vec<u8> = row.get("client_secret");

        let secret = Secret {
            id: row.get("id"),
            application_id: row.get("application_id"),
            client_id: hex::encode(client_id),
            client_secret: hex::encode(client_secret),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        };

        secrets.push(secret);
    }

    Ok(secrets)
}

pub fn remove_secret<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
    secret_id: i64,
) -> Result<Secret, ModelError> {
    let stmt = r#"
    DELETE
    FROM sso.application_secrets
    WHERE id = $1 AND application_id = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&secret_id, &application_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        let client_id: Vec<u8> = row.get("client_id");
        let client_secret: Vec<u8> = row.get("client_secret");

        Ok(Secret {
            id: row.get("id"),
            application_id: row.get("application_id"),
            client_id: hex::encode(client_id),
            client_secret: hex::encode(client_secret),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scope {
    pub id: i64,
    pub application_id: i64,
    pub name: String,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create_scope<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
    name: &str,
    description: &str,
) -> Result<Scope, ModelError> {
    let stmt = r#"
    INSERT INTO sso.scopes(application_id, name, description)
    VALUES ($1, $2, $3)
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&application_id, &name, &description])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Scope {
            id: row.get("id"),
            application_id: row.get("application_id"),
            name: row.get("name"),
            description: row.get("description"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}

pub fn select_scopes<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
) -> Result<Vec<Scope>, ModelError> {
    let stmt = r#"
    SELECT id,
           application_id,
           name,
           description,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.scopes
    WHERE application_id = $1
    "#;
    let rows = pg_conn
        .query(stmt, &[&application_id])
        .map_err(|err| ModelError::Database(err))?;

    let mut scopes = vec![];
    for row in &rows {
        let scope = Scope {
            id: row.get("id"),
            application_id: row.get("application_id"),
            name: row.get("name"),
            description: row.get("description"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        };

        scopes.push(scope);
    }

    Ok(scopes)
}

pub fn remove_scope<T: GenericConnection>(
    pg_conn: &T,
    application_id: i64,
    scope_id: i64,
) -> Result<Scope, ModelError> {
    let stmt = r#"
    DELETE
    FROM sso.scopes
    WHERE id = $1 AND application_id = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&scope_id, &application_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Scope {
            id: row.get("id"),
            application_id: row.get("application_id"),
            name: row.get("name"),
            description: row.get("description"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
    }
}
