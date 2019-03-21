use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum PreferenceKey {
    SIGNIN_TOTP = 0,
    SIGNIN_WEBAUTHN = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preference {
    pub key: i32,
    pub enabled: bool,
    pub details: Option<Value>,
}

pub fn create<C: GenericConnection>(
    pg_conn: &C,
    user_id: i64,
    preference: &Preference,
) -> Result<Preference, ModelError> {
    let stmt = r#"
    INSERT INTO sso.user_preferences(user_id, preference_key, enabled, details)
    VALUES($1, $2, $3, $4)
    ON CONFLICT ON CONSTRAINT user_preferences_unique_key
    DO UPDATE SET updated_time = now(), enabled = $3, details = $4
    RETURNING *
    "#;

    let rows = pg_conn.query(
        &stmt,
        &[
            &user_id,
            &preference.key,
            &preference.enabled,
            &preference.details,
        ],
    )?;

    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        let row = rows.get(0);
        let preference = Preference {
            key: row.get("preference_key"),
            enabled: row.get("enabled"),
            details: row.get("details"),
        };

        Ok(preference)
    }
}

pub fn select<C: GenericConnection>(
    pg_conn: &C,
    user_id: i64,
) -> Result<Vec<Preference>, ModelError> {
    let stmt = r#"
    SELECT user_preferences.preference_key, user_preferences.enabled, user_preferences.details
    FROM sso.preferences as preferences
    LEFT JOIN sso.user_preferences as user_preferences ON preferences.key = user_preferences.preference_key
    WHERE user_id = $1
    "#;
    let rows = pg_conn.query(&stmt, &[&user_id])?;

    let preferences = rows
        .iter()
        .map(|row| {
            let preference = Preference {
                key: row.get("preference_key"),
                enabled: row.get("enabled"),
                details: row.get("details"),
            };

            preference
        })
        .collect::<Vec<Preference>>();

    return Ok(preferences);
}
