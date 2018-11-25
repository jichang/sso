use super::application::{Application, Scope};
use super::Error as ModelError;
use chrono::{DateTime, Utc};
use hex;
use postgres::GenericConnection;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    pub id: i64,
    pub user_id: i64,
    pub open_id: Uuid,
    pub client_app: Application,
    pub server_app: Application,
    pub scope: Scope,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    open_id: &Uuid,
    server_id: &Vec<u8>,
    client_id: &Vec<u8>,
    scope: &str,
) -> Result<Authorization, ModelError> {
    let trans = pg_conn.transaction()?;
    let stmt = r#"
    INSERT INTO sso.authorizations(user_id, open_id, server_id, client_id, scope_id)
    VALUES (
        $1,
        $2,
        (SELECT id FROM sso.applications WHERE client_id = $3),
        (SELECT id FROM sso.applications WHERE client_id = $4),
        (SELECT id FROM sso.scopes WHERE name = $5)
    )
    ON CONFLICT ON CONSTRAINT authorizations_unique_key DO UPDATE SET updated_time = now()
    RETURNING *
    "#;

    let rows = trans.query(&stmt, &[&user_id, &open_id, &server_id, &client_id, &scope])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let authorization_id: i64 = row.get("id");

        let stmt = r#"
        SELECT authorizations.id as authorization_id,
               authorizations.user_id as authorization_user_id,
               authorizations.open_id as authorization_open_id,
               authorizations.created_time as authorization_created_time,
               authorizations.updated_time as authorization_updated_time,
               authorizations.removed_time as authorization_removed_time,
               authorizations.status as authorization_status,
               client_apps.id as client_app_id,
               client_apps.user_id as client_app_user_id,
               client_apps.name as client_app_name,
               client_apps.website_uri as client_app_website_uri,
               client_apps.client_id as client_app_client_id,
               client_apps.callback_uri as client_app_callback_uri,
               client_apps.created_time as client_app_created_time,
               client_apps.updated_time as client_app_updated_time,
               client_apps.removed_time as client_app_removed_time,
               client_apps.status as client_app_status,
               server_apps.id as server_app_id,
               server_apps.user_id as server_app_user_id,
               server_apps.name as server_app_name,
               server_apps.website_uri as server_app_website_uri,
               server_apps.client_id as server_app_client_id,
               server_apps.callback_uri as server_app_callback_uri,
               server_apps.created_time as server_app_created_time,
               server_apps.updated_time as server_app_updated_time,
               server_apps.removed_time as server_app_removed_time,
               server_apps.status as server_app_status,
               scopes.id as scope_id,
               scopes.application_id as scope_application_id,
               scopes.name as scope_name,
               scopes.description as scope_description,
               scopes.created_time as scope_created_time,
               scopes.updated_time as scope_updated_time,
               scopes.removed_time as scope_removed_time,
               scopes.status as scope_status
        FROM sso.authorizations as authorizations
        LEFT JOIN sso.applications as client_apps ON authorizations.client_id = client_apps.id
        LEFT JOIN sso.applications as server_apps ON authorizations.server_id = server_apps.id
        LEFT JOIN sso.scopes as scopes ON authorizations.scope_id = scopes.id
        WHERE authorizations.id = $1
        "#;

        let rows = trans.query(&stmt, &[&authorization_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            trans.set_commit();
            let row = rows.get(0);

            let client_app_client_id: Vec<u8> = row.get("client_app_client_id");
            let server_app_client_id: Vec<u8> = row.get("server_app_client_id");

            Ok(Authorization {
                id: row.get("authorization_id"),
                user_id: row.get("authorization_user_id"),
                open_id: row.get("authorization_open_id"),
                client_app: Application {
                    id: row.get("client_app_id"),
                    user_id: row.get("client_app_user_id"),
                    name: row.get("client_app_name"),
                    website_uri: row.get("client_app_website_uri"),
                    client_id: hex::encode(client_app_client_id),
                    client_secret: None,
                    callback_uri: row.get("client_app_callback_uri"),
                    created_time: row.get("client_app_created_time"),
                    updated_time: row.get("client_app_updated_time"),
                    removed_time: row.get("client_app_removed_time"),
                    status: row.get("client_app_status"),
                },
                server_app: Application {
                    id: row.get("server_app_id"),
                    user_id: row.get("server_app_user_id"),
                    name: row.get("server_app_name"),
                    website_uri: row.get("server_app_website_uri"),
                    client_id: hex::encode(server_app_client_id),
                    client_secret: None,
                    callback_uri: row.get("server_app_callback_uri"),
                    created_time: row.get("server_app_created_time"),
                    updated_time: row.get("server_app_updated_time"),
                    removed_time: row.get("server_app_removed_time"),
                    status: row.get("server_app_status"),
                },
                scope: Scope {
                    id: row.get("scope_id"),
                    application_id: row.get("scope_application_id"),
                    name: row.get("scope_name"),
                    description: row.get("scope_description"),
                    created_time: row.get("scope_created_time"),
                    updated_time: row.get("scope_updated_time"),
                    removed_time: row.get("scope_removed_time"),
                    status: row.get("scope_status"),
                },
                created_time: row.get("authorization_created_time"),
                updated_time: row.get("authorization_updated_time"),
                removed_time: row.get("authorization_removed_time"),
                status: row.get("authorization_status"),
            })
        }
    }
}

pub fn select<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
) -> Result<Vec<Authorization>, ModelError> {
    let stmt = r#"
    SELECT authorizations.id as authorization_id,
           authorizations.user_id as authorization_user_id,
           authorizations.open_id as authorization_open_id,
           authorizations.created_time as authorization_created_time,
           authorizations.updated_time as authorization_updated_time,
            authorizations.removed_time as authorization_removed_time,
            authorizations.status as authorization_status,
            client_apps.id as client_app_id,
            client_apps.user_id as client_app_user_id,
            client_apps.name as client_app_name,
            client_apps.website_uri as client_app_website_uri,
            client_apps.client_id as client_app_client_id,
            client_apps.callback_uri as client_app_callback_uri,
            client_apps.created_time as client_app_created_time,
            client_apps.updated_time as client_app_updated_time,
            client_apps.removed_time as client_app_removed_time,
            client_apps.status as client_app_status,
            server_apps.id as server_app_id,
            server_apps.user_id as server_app_user_id,
            server_apps.name as server_app_name,
            server_apps.website_uri as server_app_website_uri,
            server_apps.client_id as server_app_client_id,
            server_apps.callback_uri as server_app_callback_uri,
            server_apps.created_time as server_app_created_time,
            server_apps.updated_time as server_app_updated_time,
            server_apps.removed_time as server_app_removed_time,
            server_apps.status as server_app_status,
            scopes.id as scope_id,
            scopes.application_id as scope_application_id,
            scopes.name as scope_name,
            scopes.description as scope_description,
            scopes.created_time as scope_created_time,
            scopes.updated_time as scope_updated_time,
            scopes.removed_time as scope_removed_time,
            scopes.status as scope_status
    FROM sso.authorizations as authorizations
    LEFT JOIN sso.applications as client_apps ON authorizations.client_id = client_apps.id
    LEFT JOIN sso.scopes as scopes ON authorizations.scope_id = scopes.id
    LEFT JOIN sso.applications as server_apps ON scopes.application_id = server_apps.id
    WHERE authorizations.user_id = $1
    "#;
    let rows = pg_conn.query(stmt, &[&user_id])?;

    let authorizations = rows
        .iter()
        .map(|row| {
            let client_app_client_id: Vec<u8> = row.get("client_app_client_id");
            let server_app_client_id: Vec<u8> = row.get("server_app_client_id");

            Authorization {
                id: row.get("authorization_id"),
                user_id: row.get("authorization_user_id"),
                open_id: row.get("authorization_open_id"),
                client_app: Application {
                    id: row.get("client_app_id"),
                    user_id: row.get("client_app_user_id"),
                    name: row.get("client_app_name"),
                    website_uri: row.get("client_app_website_uri"),
                    client_id: hex::encode(client_app_client_id),
                    client_secret: None,
                    callback_uri: row.get("client_app_callback_uri"),
                    created_time: row.get("client_app_created_time"),
                    updated_time: row.get("client_app_updated_time"),
                    removed_time: row.get("client_app_removed_time"),
                    status: row.get("client_app_status"),
                },
                server_app: Application {
                    id: row.get("server_app_id"),
                    user_id: row.get("server_app_user_id"),
                    name: row.get("server_app_name"),
                    website_uri: row.get("server_app_website_uri"),
                    client_id: hex::encode(server_app_client_id),
                    client_secret: None,
                    callback_uri: row.get("server_app_callback_uri"),
                    created_time: row.get("server_app_created_time"),
                    updated_time: row.get("server_app_updated_time"),
                    removed_time: row.get("server_app_removed_time"),
                    status: row.get("server_app_status"),
                },
                scope: Scope {
                    id: row.get("scope_id"),
                    application_id: row.get("scope_application_id"),
                    name: row.get("scope_name"),
                    description: row.get("scope_description"),
                    created_time: row.get("scope_created_time"),
                    updated_time: row.get("scope_updated_time"),
                    removed_time: row.get("scope_removed_time"),
                    status: row.get("scope_status"),
                },
                created_time: row.get("authorization_created_time"),
                updated_time: row.get("authorization_updated_time"),
                removed_time: row.get("authorization_removed_time"),
                status: row.get("authorization_status"),
            }
        })
        .collect::<Vec<Authorization>>();

    Ok(authorizations)
}

pub fn remove<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    authorization_id: i64,
) -> Result<Authorization, ModelError> {
    let trans = pg_conn.transaction()?;
    let authorization = select_one(&trans, authorization_id)?;
    println!("{:?}", authorization);
    let stmt = r#"
    DELETE
    FROM sso.authorizations
    WHERE id = $1 AND user_id = $2
    RETURNING *
    "#;

    let rows = trans.query(&stmt, &[&authorization_id, &user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        trans.set_commit();

        Ok(authorization)
    }
}

pub fn select_one<T: GenericConnection>(
    pg_conn: &T,
    authorization_id: i64,
) -> Result<Authorization, ModelError> {
    let stmt = r#"
        SELECT authorizations.id as authorization_id,
               authorizations.user_id as authorization_user_id,
               authorizations.open_id as authorization_open_id,
               authorizations.created_time as authorization_created_time,
               authorizations.updated_time as authorization_updated_time,
               authorizations.removed_time as authorization_removed_time,
               authorizations.status as authorization_status,
               client_apps.id as client_app_id,
               client_apps.user_id as client_app_user_id,
               client_apps.name as client_app_name,
               client_apps.website_uri as client_app_website_uri,
               client_apps.client_id as client_app_client_id,
               client_apps.callback_uri as client_app_callback_uri,
               client_apps.created_time as client_app_created_time,
               client_apps.updated_time as client_app_updated_time,
               client_apps.removed_time as client_app_removed_time,
               client_apps.status as client_app_status,
               server_apps.id as server_app_id,
               server_apps.user_id as server_app_user_id,
               server_apps.name as server_app_name,
               server_apps.website_uri as server_app_website_uri,
               server_apps.client_id as server_app_client_id,
               server_apps.callback_uri as server_app_callback_uri,
               server_apps.created_time as server_app_created_time,
               server_apps.updated_time as server_app_updated_time,
               server_apps.removed_time as server_app_removed_time,
               server_apps.status as server_app_status,
               scopes.id as scope_id,
               scopes.application_id as scope_application_id,
               scopes.name as scope_name,
               scopes.description as scope_description,
               scopes.created_time as scope_created_time,
               scopes.updated_time as scope_updated_time,
               scopes.removed_time as scope_removed_time,
               scopes.status as scope_status
        FROM sso.authorizations as authorizations
        LEFT JOIN sso.applications as client_apps ON authorizations.client_id = client_apps.id
        LEFT JOIN sso.scopes as scopes ON authorizations.scope_id = scopes.id
        LEFT JOIN sso.applications as server_apps ON scopes.application_id = server_apps.id
        WHERE authorizations.id = $1
    "#;

    let rows = pg_conn.query(&stmt, &[&authorization_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let client_app_client_id: Vec<u8> = row.get("client_app_client_id");
        let server_app_client_id: Vec<u8> = row.get("server_app_client_id");

        Ok(Authorization {
            id: row.get("authorization_id"),
            user_id: row.get("authorization_user_id"),
            open_id: row.get("authorization_open_id"),
            client_app: Application {
                id: row.get("client_app_id"),
                user_id: row.get("client_app_user_id"),
                name: row.get("client_app_name"),
                website_uri: row.get("client_app_website_uri"),
                client_id: hex::encode(client_app_client_id),
                client_secret: None,
                callback_uri: row.get("client_app_callback_uri"),
                created_time: row.get("client_app_created_time"),
                updated_time: row.get("client_app_updated_time"),
                removed_time: row.get("client_app_removed_time"),
                status: row.get("client_app_status"),
            },
            server_app: Application {
                id: row.get("server_app_id"),
                user_id: row.get("server_app_user_id"),
                name: row.get("server_app_name"),
                website_uri: row.get("server_app_website_uri"),
                client_id: hex::encode(server_app_client_id),
                client_secret: None,
                callback_uri: row.get("server_app_callback_uri"),
                created_time: row.get("server_app_created_time"),
                updated_time: row.get("server_app_updated_time"),
                removed_time: row.get("server_app_removed_time"),
                status: row.get("server_app_status"),
            },
            scope: Scope {
                id: row.get("scope_id"),
                application_id: row.get("scope_application_id"),
                name: row.get("scope_name"),
                description: row.get("scope_description"),
                created_time: row.get("scope_created_time"),
                updated_time: row.get("scope_updated_time"),
                removed_time: row.get("scope_removed_time"),
                status: row.get("scope_status"),
            },
            created_time: row.get("authorization_created_time"),
            updated_time: row.get("authorization_updated_time"),
            removed_time: row.get("authorization_removed_time"),
            status: row.get("authorization_status"),
        })
    }
}

pub fn verify<T: GenericConnection>(
    pg_conn: &T,
    authorization_id: i64,
    client_id: &Vec<u8>,
    client_secret: &Vec<u8>,
) -> Result<Uuid, ModelError> {
    let stmt = r#"
        SELECT authorizations.open_id
        FROM sso.authorizations as authorizations
        LEFT JOIN sso.applications as client_apps ON authorizations.client_id = client_apps.id
        WHERE authorizations.id = $1
          AND client_apps.client_id = $2
          AND client_apps.client_secret = $3
    "#;

    let rows = pg_conn.query(&stmt, &[&authorization_id, &client_id, &client_secret])?;
    if rows.len() != 1 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);
        let open_id: Uuid = row.get("open_id");
        Ok(open_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationPreview {
    pub client_app: Application,
    pub server_app: Application,
    pub scope: Scope,
}

pub fn preview<T: GenericConnection>(
    pg_conn: &T,
    server_id: &Vec<u8>,
    client_id: &Vec<u8>,
    scope_name: &str,
) -> Result<AuthorizationPreview, ModelError> {
    let trans = pg_conn.transaction()?;
    let stmt = r#"
        SELECT client_apps.id as client_app_id,
               client_apps.user_id as client_app_user_id,
               client_apps.name as client_app_name,
               client_apps.website_uri as client_app_website_uri,
               client_apps.client_id as client_app_client_id,
               client_apps.callback_uri as client_app_callback_uri,
               client_apps.created_time as client_app_created_time,
               client_apps.updated_time as client_app_updated_time,
               client_apps.removed_time as client_app_removed_time,
               client_apps.status as client_app_status
        FROM sso.applications as client_apps
        WHERE client_apps.client_id = $1
    "#;

    let rows = trans.query(&stmt, &[&client_id])?;
    if rows.len() != 1 {
        println!("no client app found");
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let client_app_client_id: Vec<u8> = row.get("client_app_client_id");

        let client_app = Application {
            id: row.get("client_app_id"),
            user_id: row.get("client_app_user_id"),
            name: row.get("client_app_name"),
            website_uri: row.get("client_app_website_uri"),
            client_id: hex::encode(client_app_client_id),
            client_secret: None,
            callback_uri: row.get("client_app_callback_uri"),
            created_time: row.get("client_app_created_time"),
            updated_time: row.get("client_app_updated_time"),
            removed_time: row.get("client_app_removed_time"),
            status: row.get("client_app_status"),
        };

        let stmt = r#"
        SELECT server_apps.id as server_app_id,
               server_apps.user_id as server_app_user_id,
               server_apps.name as server_app_name,
               server_apps.website_uri as server_app_website_uri,
               server_apps.client_id as server_app_client_id,
               server_apps.callback_uri as server_app_callback_uri,
               server_apps.created_time as server_app_created_time,
               server_apps.updated_time as server_app_updated_time,
               server_apps.removed_time as server_app_removed_time,
               server_apps.status as server_app_status,
               scopes.id as scope_id,
               scopes.application_id as scope_application_id,
               scopes.name as scope_name,
               scopes.description as scope_description,
               scopes.created_time as scope_created_time,
               scopes.updated_time as scope_updated_time,
               scopes.removed_time as scope_removed_time,
               scopes.status as scope_status
        FROM sso.applications as server_apps
        LEFT JOIN sso.scopes as scopes ON scopes.application_id = server_apps.id
        WHERE scopes.name = $1 AND server_apps.client_id = $2
        "#;

        let rows = trans.query(&stmt, &[&scope_name, &server_id])?;
        if rows.len() != 1 {
            println!("no server app found");
            Err(ModelError::Unknown)
        } else {
            let row = rows.get(0);
            let server_app_client_id: Vec<u8> = row.get("server_app_client_id");
            let server_app = Application {
                id: row.get("server_app_id"),
                user_id: row.get("server_app_user_id"),
                name: row.get("server_app_name"),
                website_uri: row.get("server_app_website_uri"),
                client_id: hex::encode(server_app_client_id),
                client_secret: None,
                callback_uri: row.get("server_app_callback_uri"),
                created_time: row.get("server_app_created_time"),
                updated_time: row.get("server_app_updated_time"),
                removed_time: row.get("server_app_removed_time"),
                status: row.get("server_app_status"),
            };
            let scope = Scope {
                id: row.get("scope_id"),
                application_id: row.get("scope_application_id"),
                name: row.get("scope_name"),
                description: row.get("scope_description"),
                created_time: row.get("scope_created_time"),
                updated_time: row.get("scope_updated_time"),
                removed_time: row.get("scope_removed_time"),
                status: row.get("scope_status"),
            };

            trans.set_commit();

            Ok(AuthorizationPreview {
                client_app: client_app,
                server_app: server_app,
                scope: scope,
            })
        }
    }
}
