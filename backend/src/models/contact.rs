use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;

pub enum ContactTypeId {
    Email = 1,
    Phone = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactType {
    id: i32,
    name: String,
    created_time: DateTime<Utc>,
    updated_time: Option<DateTime<Utc>>,
    removed_time: Option<DateTime<Utc>>,
    status: i32,
}

pub fn select_types<T: GenericConnection>(pg_conn: &T) -> Result<Vec<ContactType>, ModelError> {
    let stmt = r#"
    SELECT id, name, created_time, updated_time, removed_time, status
    FROM sso.contact_types
    "#;
    let rows = pg_conn.query(stmt, &[])?;

    let contact_types = rows
        .iter()
        .map(|row| ContactType {
            id: row.get("id"),
            name: row.get("name"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<ContactType>>();

    Ok(contact_types)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub user_id: i64,
    #[serde(rename = "type")]
    pub typ: ContactType,
    pub identity: String,
    created_time: DateTime<Utc>,
    verified_time: Option<DateTime<Utc>>,
    updated_time: Option<DateTime<Utc>>,
    removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    type_id: i32,
    identity: &str,
) -> Result<Contact, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    INSERT INTO sso.contacts(user_id, type_id, identity)
    VALUES ($1, $2, $3)
    RETURNING *
    "#;
    let rows = trans.query(&stmt, &[&user_id, &type_id, &identity])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let contact_row = rows.get(0);
        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.contact_types
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&type_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let type_row = rows.get(0);

            trans.set_commit();

            Ok(Contact {
                id: contact_row.get("id"),
                user_id: contact_row.get("user_id"),
                typ: ContactType {
                    id: type_row.get("id"),
                    name: type_row.get("name"),
                    created_time: type_row.get("created_time"),
                    updated_time: type_row.get("updated_time"),
                    removed_time: type_row.get("removed_time"),
                    status: type_row.get("status"),
                },
                identity: contact_row.get("identity"),
                created_time: contact_row.get("created_time"),
                updated_time: contact_row.get("updated_time"),
                verified_time: contact_row.get("verified_time"),
                removed_time: contact_row.get("removed_time"),
                status: contact_row.get("status"),
            })
        }
    }
}

pub fn verify<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    contact_id: i64,
) -> Result<Contact, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    UPDATE sso.contacts
    SET status = 1, verified_time = now()
    WHERE id = $1 AND user_id = $2
    RETURNING *
    "#;

    let rows = trans.query(&stmt, &[&contact_id, &user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let contact_row = rows.get(0);
        let type_id: i32 = contact_row.get("type_id");
        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.contact_types
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&type_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let type_row = rows.get(0);

            trans.set_commit();

            Ok(Contact {
                id: contact_row.get("id"),
                user_id: contact_row.get("user_id"),
                typ: ContactType {
                    id: type_row.get("id"),
                    name: type_row.get("name"),
                    created_time: type_row.get("created_time"),
                    updated_time: type_row.get("updated_time"),
                    removed_time: type_row.get("removed_time"),
                    status: type_row.get("status"),
                },
                identity: contact_row.get("identity"),
                created_time: contact_row.get("created_time"),
                updated_time: contact_row.get("updated_time"),
                verified_time: contact_row.get("verified_time"),
                removed_time: contact_row.get("removed_time"),
                status: contact_row.get("status"),
            })
        }
    }
}

pub fn select<T: GenericConnection>(pg_conn: &T, user_id: i64) -> Result<Vec<Contact>, ModelError> {
    let stmt = r#"
    SELECT contacts.id as contact_id,
           contacts.user_id as contact_user_id,
           contacts.identity as contact_identity,
           contacts.created_time as contact_created_time,
           contacts.verified_time as contact_verified_time,
           contacts.updated_time as contact_updated_time,
           contacts.removed_time as contact_removed_time,
           contacts.status as contact_status,
           contact_types.id as type_id,
           contact_types.name as type_name,
           contact_types.created_time as type_created_time,
           contact_types.updated_time as type_updated_time,
           contact_types.removed_time as type_removed_time,
           contact_types.status as type_status
    FROM sso.contacts as contacts
    LEFT JOIN sso.contact_types as contact_types ON contact_types.id = contacts.type_id
    WHERE contacts.user_id = $1
    "#;
    let rows = pg_conn.query(stmt, &[&user_id])?;

    let contacts = rows
        .iter()
        .map(|row| Contact {
            id: row.get("contact_id"),
            user_id: row.get("contact_user_id"),
            typ: ContactType {
                id: row.get("type_id"),
                name: row.get("type_name"),
                created_time: row.get("type_created_time"),
                updated_time: row.get("type_updated_time"),
                removed_time: row.get("type_removed_time"),
                status: row.get("type_status"),
            },
            identity: row.get("contact_identity"),
            created_time: row.get("contact_created_time"),
            updated_time: row.get("contact_updated_time"),
            verified_time: row.get("contact_verified_time"),
            removed_time: row.get("contact_removed_time"),
            status: row.get("contact_status"),
        })
        .collect::<Vec<Contact>>();

    Ok(contacts)
}

pub fn select_one<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    contact_id: i64,
) -> Result<Contact, ModelError> {
    let stmt = r#"
    SELECT contacts.id as contact_id,
           contacts.user_id as contact_user_d,
           contacts.identity as contact_identity,
           contacts.created_time as contact_created_time,
           contacts.verified_time as contact_verified_time,
           contacts.updated_time as contact_updated_time,
           contacts.removed_time as contact_removed_time,
           contacts.status as contact_status,
           contact_types.id as type_id,
           contact_types.name as type_name,
           contact_types.created_time as type_created_time,
           contact_types.verified_time as type_verified_time,
           contact_types.updated_time as type_updated_time,
           contact_types.removed_time as type_removed_time,
           contact_types.status as type_status
    FROM sso.contacts as contacts
    LEFT JOIN sso.contact_types as contact_types ON contact_types.id = contacts.type_id
    WHERE contacts.user_id = $1 AND contacts.id = $2
    "#;
    let rows = pg_conn.query(stmt, &[&user_id, &contact_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);

        Ok(Contact {
            id: row.get("contact_id"),
            user_id: row.get("contact_user_id"),
            typ: ContactType {
                id: row.get("type_id"),
                name: row.get("type_name"),
                created_time: row.get("type_created_time"),
                updated_time: row.get("type_updated_time"),
                removed_time: row.get("type_removed_time"),
                status: row.get("type_status"),
            },
            identity: row.get("contact_identity"),
            created_time: row.get("contact_created_time"),
            updated_time: row.get("contact_updated_time"),
            verified_time: row.get("contact_verified_time"),
            removed_time: row.get("contact_removed_time"),
            status: row.get("contact_status"),
        })
    }
}

pub fn remove<T: GenericConnection>(
    pg_conn: &T,
    contact_id: i64,
    user_id: i64,
) -> Result<Contact, ModelError> {
    let trans = pg_conn.transaction()?;
    let stmt = r#"
    DELETE
    FROM sso.contacts
    WHERE id = $1 AND user_id = $2
    RETURNING *
    "#;

    let rows = trans.query(&stmt, &[&contact_id, &user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let contact_row = rows.get(0);
        let type_id: i32 = contact_row.get("type_id");
        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.contact_types
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&type_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let type_row = rows.get(0);

            trans.set_commit();

            Ok(Contact {
                id: contact_row.get("id"),
                user_id: contact_row.get("user_id"),
                typ: ContactType {
                    id: type_row.get("id"),
                    name: type_row.get("name"),
                    created_time: type_row.get("created_time"),
                    updated_time: type_row.get("updated_time"),
                    removed_time: type_row.get("removed_time"),
                    status: type_row.get("status"),
                },
                identity: contact_row.get("identity"),
                created_time: contact_row.get("created_time"),
                updated_time: contact_row.get("updated_time"),
                verified_time: contact_row.get("verified_time"),
                removed_time: contact_row.get("removed_time"),
                status: contact_row.get("status"),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::ContactTypeId;
    use postgres::{Connection, TlsMode};

    #[test]
    pub fn test_contact_types_select() {
        let conn =
            Connection::connect("postgres://feblr:feblr@localhost/feblr", TlsMode::None).unwrap();
        let contact_types = super::select_types(&conn).unwrap();

        assert_eq!(contact_types.len(), 2);

        contact_types
            .into_iter()
            .for_each(|contact_type| match contact_type {
                _ if contact_type.id == ContactTypeId::Email as i32 => {
                    assert_eq!(contact_type.name, "email");
                    assert_eq!(contact_type.status, 1);
                }
                _ if contact_type.id == ContactTypeId::Phone as i32 => {
                    assert_eq!(contact_type.name, "phone");
                    assert_eq!(contact_type.status, 0);
                }
                _ => {
                    panic!("unknown contact type");
                }
            });
    }
}
