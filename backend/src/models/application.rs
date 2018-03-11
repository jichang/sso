use std::convert::From;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use super::Error as ModelError;
use url::Url;
use hex;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientSecret {
    Plaintext(String),
    Ciphertext(String),
}

impl ClientSecret {
    pub fn plaintext(value: String) -> ClientSecret {
        ClientSecret::Plaintext(value)
    }

    pub fn ciphertext(value: String) -> ClientSecret {
        ClientSecret::Ciphertext(value)
    }

    pub fn value(&self) -> &str {
        match self {
            &ClientSecret::Plaintext(ref secret) => secret,
            &ClientSecret::Ciphertext(ref secret) => secret,
        }
    }

    pub fn is_ciphertext(secret: &Self) -> bool {
        match secret {
            &ClientSecret::Plaintext(_) => false,
            &ClientSecret::Ciphertext(_) => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub website_uri: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "ClientSecret::is_ciphertext")]
    pub client_secret: ClientSecret,
    pub callback_uri: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    name: &str,
    website_uri: &str,
    client_id: &Vec<u8>,
    client_secret: &Vec<u8>,
    callback_uri: &str,
    create_secret: fn(String) -> ClientSecret,
) -> Result<Application, ModelError> {
    let website_uri = Url::parse(website_uri).map_err(|err| {
        ModelError::InvalidParam(String::from("website_uri"), Box::new(err))
    })?;
    let callback_uri = Url::parse(callback_uri).map_err(|err| {
        ModelError::InvalidParam(String::from("callback_uri"), Box::new(err))
    })?;

    let stmt = r#"
    INSERT INTO sso.applications(user_id, name, website_uri, client_id, client_secret, callback_uri)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING *
    "#;

    let rows = pg_conn.query(
        &stmt,
        &[
            &user_id,
            &name,
            &website_uri.as_str(),
            &client_id,
            &client_secret,
            &callback_uri.as_str(),
        ],
    )?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let client_secret: Vec<u8> = row.get("client_secret");

        Ok(Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            client_id: hex::encode(client_id),
            client_secret: create_secret(hex::encode(client_secret)),
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
    create_secret: fn(String) -> ClientSecret,
) -> Result<Vec<Application>, ModelError> {
    let stmt = r#"
    SELECT id,
           user_id,
           name,
           website_uri,
           client_id,
           client_secret,
           callback_uri,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.applications
    WHERE user_id = $1
    "#;
    let rows = pg_conn.query(stmt, &[&user_id]).map_err(|err| {
        ModelError::Database(err)
    })?;

    let mut applications = vec![];
    for row in &rows {
        let client_id: Vec<u8> = row.get("client_id");
        let client_secret: Vec<u8> = row.get("client_secret");

        let application = Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            client_id: hex::encode(client_id),
            client_secret: create_secret(hex::encode(client_secret)),
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
    create_secret: fn(String) -> ClientSecret,
) -> Result<Application, ModelError> {
    let stmt = r#"
    SELECT id,
           user_id,
           name,
           website_uri,
           client_id,
           client_secret,
           callback_uri,
           created_time,
           updated_time,
           removed_time,
           status
    FROM sso.applications
    WHERE client_id = $1
    "#;
    let rows = pg_conn.query(stmt, &[&client_id]).map_err(|err| {
        ModelError::Database(err)
    })?;

    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let client_id: Vec<u8> = row.get("client_id");
        let client_secret: Vec<u8> = row.get("client_secret");

        let application = Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            client_id: hex::encode(client_id),
            client_secret: create_secret(hex::encode(client_secret)),
            callback_uri: row.get("callback_uri"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        };
        Ok(application)
    }
}

pub fn remove<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    application_id: i64,
    create_secret: fn(String) -> ClientSecret,
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
        let client_id: Vec<u8> = row.get("client_id");
        let client_secret: Vec<u8> = row.get("client_secret");

        Ok(Application {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            website_uri: row.get("website_uri"),
            client_id: hex::encode(client_id),
            client_secret: create_secret(hex::encode(client_secret)),
            callback_uri: row.get("callback_uri"),
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

    let rows = pg_conn.query(
        &stmt,
        &[&application_id, &name, &description],
    )?;
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
    let rows = pg_conn.query(stmt, &[&application_id]).map_err(|err| {
        ModelError::Database(err)
    })?;

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
