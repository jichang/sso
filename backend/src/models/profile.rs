use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use super::Error as ModelError;

pub enum GenderId {
    Male = 1,
    Female = 2,
    Others = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gender {
    pub id: i32,
    pub name: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn select_genders<T: GenericConnection>(pg_conn: &T) -> Result<Vec<Gender>, ModelError> {
    let stmt = r#"
    SELECT id, name, created_time, updated_time, removed_time, status
    FROM sso.genders
    "#;
    let rows = pg_conn.query(stmt, &[])?;

    let genders = rows.iter()
        .map(|row| Gender {
            id: row.get("id"),
            name: row.get("name"),
            created_time: row.get("created_time"),
            updated_time: row.get("updated_time"),
            removed_time: row.get("removed_time"),
            status: row.get("status"),
        })
        .collect::<Vec<Gender>>();

    Ok(genders)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i64,
    pub user_id: i64,
    pub gender: Gender,
    pub name: String,
    pub birthday: DateTime<Utc>,
    pub introduction: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
    pub removed_time: Option<DateTime<Utc>>,
    pub status: i32,
}

pub fn create<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    gender_id: i32,
    name: &str,
    birthday: &DateTime<Utc>,
    introduction: &str,
) -> Result<Profile, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    INSERT INTO sso.profiles(user_id, gender_id, name, birthday, introduction)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING *
    "#;

    let rows = trans.query(
        &stmt,
        &[&user_id, &gender_id, &name, &birthday, &introduction],
    )?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let profile_row = rows.get(0);
        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.genders
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&gender_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let gender_row = rows.get(0);

            trans.set_commit();

            Ok(Profile {
                id: profile_row.get("id"),
                user_id: profile_row.get("user_id"),
                gender: Gender {
                    id: gender_row.get("id"),
                    name: gender_row.get("name"),
                    created_time: gender_row.get("created_time"),
                    updated_time: gender_row.get("updated_time"),
                    removed_time: gender_row.get("removed_time"),
                    status: gender_row.get("status"),
                },
                name: profile_row.get("name"),
                birthday: profile_row.get("birthday"),
                introduction: profile_row.get("introduction"),
                created_time: profile_row.get("created_time"),
                updated_time: profile_row.get("updated_time"),
                removed_time: profile_row.get("removed_time"),
                status: profile_row.get("status"),
            })
        }
    }
}

pub fn select<T: GenericConnection>(pg_conn: &T, user_id: i64) -> Result<Profile, ModelError> {
    let stmt = r#"
    SELECT profiles.id as profile_id,
           profiles.user_id as profile_user_id,
           profiles.name as profile_name,
           profiles.birthday as profile_birthday,
           profiles.introduction as profile_introduction,
           profiles.created_time as profile_created_time,
           profiles.updated_time as profile_updated_time,
           profiles.removed_time as profile_removed_time,
           profiles.status as profile_status,
           genders.id as gender_id,
           genders.name as gender_name,
           genders.created_time as gender_created_time,
           genders.updated_time as gender_updated_time,
           genders.removed_time as gender_removed_time,
           genders.status as gender_status
    FROM sso.profiles as profiles
    JOIN sso.genders as genders ON profiles.gender_id = genders.id
    WHERE user_id = $1
    "#;

    let rows = pg_conn.query(&stmt, &[&user_id])?;
    if rows.len() != 1 {
        Err(ModelError::NotFound)
    } else {
        let row = rows.get(0);

        Ok(Profile {
            id: row.get("profile_id"),
            user_id: row.get("profile_user_id"),
            name: row.get("profile_name"),
            gender: Gender {
                id: row.get("gender_id"),
                name: row.get("gender_name"),
                created_time: row.get("gender_created_time"),
                updated_time: row.get("gender_updated_time"),
                removed_time: row.get("gender_removed_time"),
                status: row.get("gender_status"),
            },
            birthday: row.get("profile_birthday"),
            introduction: row.get("profile_introduction"),
            created_time: row.get("profile_created_time"),
            updated_time: row.get("profile_updated_time"),
            removed_time: row.get("profile_removed_time"),
            status: row.get("profile_status"),
        })
    }
}

pub fn update<T: GenericConnection>(
    pg_conn: &T,
    user_id: i64,
    gender_id: i32,
    name: &str,
    birthday: &DateTime<Utc>,
    introduction: &str,
) -> Result<Profile, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    UPDATE sso.profiles
    SET user_id = $1,
        gender_id = $2,
        name = $3,
        birthday = $4,
        introduction = $5,
        updated_time = now()
    RETURNING *
    "#;

    let rows = trans.query(
        &stmt,
        &[&user_id, &gender_id, &name, &birthday, &introduction],
    )?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let profile_row = rows.get(0);
        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.genders
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&gender_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let gender_row = rows.get(0);

            trans.set_commit();

            Ok(Profile {
                id: profile_row.get("id"),
                user_id: profile_row.get("user_id"),
                gender: Gender {
                    id: gender_row.get("id"),
                    name: gender_row.get("name"),
                    created_time: gender_row.get("created_time"),
                    updated_time: gender_row.get("updated_time"),
                    removed_time: gender_row.get("removed_time"),
                    status: gender_row.get("status"),
                },
                name: profile_row.get("name"),
                birthday: profile_row.get("birthday"),
                introduction: profile_row.get("introduction"),
                created_time: profile_row.get("created_time"),
                updated_time: profile_row.get("updated_time"),
                removed_time: profile_row.get("removed_time"),
                status: profile_row.get("status"),
            })
        }
    }
}

pub fn remove<T: GenericConnection>(pg_conn: &T, user_id: i64) -> Result<Profile, ModelError> {
    let trans = pg_conn.transaction()?;

    let stmt = r#"
    DELETE sso.profiles
    WHERE user_id = $2
    RETURNING *
    "#;

    let rows = pg_conn.query(&stmt, &[&user_id])?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let profile_row = rows.get(0);
        let gender_id: i32 = profile_row.get("gender_id");

        let stmt = r#"
        SELECT id, name, created_time, updated_time, removed_time, status
        FROM sso.genders
        WHERE id = $1
        "#;
        let rows = trans.query(&stmt, &[&gender_id])?;
        if rows.len() != 1 {
            Err(ModelError::Unknown)
        } else {
            let gender_row = rows.get(0);

            trans.set_commit();

            Ok(Profile {
                id: profile_row.get("id"),
                user_id: profile_row.get("user_id"),
                gender: Gender {
                    id: gender_row.get("id"),
                    name: gender_row.get("name"),
                    created_time: gender_row.get("created_time"),
                    updated_time: gender_row.get("updated_time"),
                    removed_time: gender_row.get("removed_time"),
                    status: gender_row.get("status"),
                },
                name: profile_row.get("name"),
                birthday: profile_row.get("birthday"),
                introduction: profile_row.get("introduction"),
                created_time: profile_row.get("created_time"),
                updated_time: profile_row.get("updated_time"),
                removed_time: profile_row.get("removed_time"),
                status: profile_row.get("status"),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::GenderId;
    use postgres::{Connection, TlsMode};

    #[test]
    pub fn test_genders_select() {
        let conn =
            Connection::connect("postgres://feblr:feblr@localhost/feblr", TlsMode::None).unwrap();
        let genders = super::select_genders(&conn).unwrap();

        assert_eq!(genders.len(), 3);

        genders
            .into_iter()
            .map(|gender| match gender {
                _ if gender.id == GenderId::Male as i32 => assert_eq!(gender.name, "male"),
                _ if gender.id == GenderId::Female as i32 => assert_eq!(gender.name, "female"),
                _ if gender.id == GenderId::Others as i32 => assert_eq!(gender.name, "others"),
                _ => panic!("unexist gender"),
            })
            .collect::<()>();
    }
}
