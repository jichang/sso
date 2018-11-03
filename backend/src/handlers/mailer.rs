use super::super::config_parser::Config;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::Error as SmtpError;
use lettre::smtp::response::Response;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransport;
use lettre::EmailTransport;
use lettre_email::error::Error as EmailError;
use lettre_email::EmailBuilder;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum MailerError {
    Smtp(SmtpError),
    Mail(EmailError),
}

impl fmt::Display for MailerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MailerError::Smtp(ref err) => err.fmt(f),
            MailerError::Mail(ref err) => err.fmt(f),
        }
    }
}

impl StdError for MailerError {
    fn description(&self) -> &str {
        match *self {
            MailerError::Smtp(_) => "can not create email",
            MailerError::Mail(_) => "can not build email",
        }
    }
}

impl From<SmtpError> for MailerError {
    fn from(err: SmtpError) -> MailerError {
        MailerError::Smtp(err)
    }
}

impl From<EmailError> for MailerError {
    fn from(err: EmailError) -> MailerError {
        MailerError::Mail(err)
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
        config.mailer.verify_link, contact_id, verify_token
    );
    let email = EmailBuilder::new()
        .to((email_addr, ""))
        .from(config.mailer.username.to_string())
        .subject("Welcome to feblr")
        .text(body)
        .build()?;

    let credentials = Credentials::new(
        config.mailer.username.to_string(),
        config.mailer.password.to_string(),
    );
    let mut mailer = SmtpTransport::simple_builder(&config.mailer.server)?
        .credentials(credentials)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .build();
    let result = mailer.send(&email);
    mailer.close();

    match result {
        Ok(res) => Ok(res),
        Err(err) => Err(MailerError::from(err)),
    }
}
