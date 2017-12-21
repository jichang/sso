use std::fmt;
use std::error::Error as StdError;
use lettre::{EmailAddress, SimpleSendableEmail};
use lettre::smtp::error::Error as SmtpError;
use lettre::smtp::SmtpTransport;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::response::Response;
use lettre::EmailTransport;
use super::super::config::Config;

#[derive(Debug)]
pub enum MailerError {
    Smtp(SmtpError),
}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MailerError::Smtp(ref err) => err.fmt(f),
        }
    }
}

impl StdError for MailerError {
    fn description(&self) -> &str {
        match *self {
            MailerError::Smtp(_) => "can not create email",
        }
    }
}

impl From<SmtpError> for MailerError {
    fn from(err: SmtpError) -> MailerError {
        MailerError::Smtp(err)
    }
}

pub fn send_token(
    config: &Config,
    contact_id: i64,
    email_addr: &str,
    verify_token: &str,
) -> Result<Response, MailerError> {
    let body = format!(
        "verify link: {}?contact_id={}&token={}",
        config.mailer.verify_link,
        contact_id,
        verify_token
    );

    let email = SimpleSendableEmail::new(
        EmailAddress::new(config.mailer.username.to_string()),
        vec![EmailAddress::new(email_addr.to_string())],
        format!("email"),
        body,
    );

    let credentials = Credentials::new(
        config.mailer.username.to_string(),
        config.mailer.password.to_string(),
    );
    let mut mailer = SmtpTransport::simple_builder(config.mailer.server.to_string())?
        .credentials(credentials)
        .smtp_utf8(true)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();
    let result = mailer.send(&email)?;
    mailer.close();

    Ok(result)
}
