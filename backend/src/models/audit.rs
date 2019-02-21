use super::Error as ModelError;
use chrono::{DateTime, Utc};
use postgres::GenericConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::IpAddr;

pub trait ActivityType {
    fn to_i32(&self) -> i32;
}

pub struct Activity<T: ActivityType, D: Serialize + Deserialize<'static>> {
    pub activity_type: T,
    pub client_addr: IpAddr,
    pub happened_time: DateTime<Utc>,
    pub details: D,
}

#[derive(Clone, Copy)]
pub struct Signin;

impl ActivityType for Signin {
    fn to_i32(&self) -> i32 {
        0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninActivityDetails {
    pub is_succeed: bool,
}

pub type SigninActivity = Activity<Signin, SigninActivityDetails>;

#[derive(Clone, Copy)]
pub struct ChangePassword;

impl ActivityType for ChangePassword {
    fn to_i32(&self) -> i32 {
        1
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordActivityDetails {
    pub is_succeed: bool,
}

pub type ChangePasswordActivity = Activity<ChangePassword, ChangePasswordActivityDetails>;

pub fn create<C: GenericConnection, T: ActivityType, D: Serialize + Deserialize<'static>>(
    pg_conn: &C,
    username: &str,
    activity: Activity<T, D>,
) -> Result<(), ModelError> {
    let details = serde_json::to_value(activity.details)?;
    let stmt = r#"
    INSERT INTO sso.audits(user_id, client_addr, type, details)
    VALUES((SELECT user_id FROM sso.accounts WHERE username = $1), $2, $3, $4)
    RETURNING *
    "#;

    let rows = pg_conn.query(
        &stmt,
        &[
            &username,
            &activity.client_addr.to_string(),
            &activity.activity_type.to_i32(),
            &details,
        ],
    )?;
    if rows.len() != 1 {
        Err(ModelError::Unknown)
    } else {
        Ok(())
    }
}

pub fn select<C: GenericConnection, T: ActivityType + Copy, D>(
    pg_conn: &C,
    username: &str,
    activity_type: T,
    happened_time: DateTime<Utc>,
) -> Result<Vec<Activity<T, D>>, ModelError>
where
    for<'de> D: serde::Deserialize<'de> + Serialize,
{
    let stmt = r#"
    SELECT happened_time, client_addr, details
    FROM sso.audits
    LEFT JOIN sso.accounts ON sso.audits.user_id = sso.accounts.user_id
    WHERE sso.accounts.username = $1 AND type = $2 AND happened_time > $3
    "#;
    let rows = pg_conn.query(&stmt, &[&username, &activity_type.to_i32(), &happened_time])?;

    let activities = rows
        .iter()
        .map(|row| {
            let client_addr: String = row.get("client_addr");
            let details: Value = row.get("details");
            let activity = Activity {
                activity_type: activity_type,
                client_addr: client_addr.parse().unwrap(),
                happened_time: row.get("happened_time"),
                details: serde_json::from_value(details).unwrap(),
            };

            activity
        })
        .collect::<Vec<Activity<T, D>>>();

    return Ok(activities);
}
