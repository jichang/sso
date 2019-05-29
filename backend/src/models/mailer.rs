use super::super::config_parser::Config;
use super::Error as ModelError;
use lettre::{SendableEmail, EmailAddress, Transport, Envelope, SmtpClient};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::error::Error as SmtpError;
use lettre::smtp::response::Response;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransport;
use lettre_email::Email;
use lettre_email::error::Error as EmailError;
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
) -> Result<Response, ModelError> {
    let body = format!(
        "verify link: {}?target_id={}&target_type=email&token={}",
        config.mailer.verify_link, contact_id, verify_token
    );
    let email = Email::builder()
        .to((email_addr, ""))
        .from(config.mailer.username.to_string())
        .subject("Welcome to feblr")
        .text(body)
        .build()
        .map_err(MailerError::Mail)?;

    let credentials = Credentials::new(
        config.mailer.username.to_string(),
        config.mailer.password.to_string(),
    );
    let mut mailer = SmtpClient::new_simple(&config.mailer.server)
        .map_err(MailerError::Smtp)?
        .credentials(credentials)
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Login)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    let response = mailer.send(email.into()).map_err(MailerError::Smtp)?;
    mailer.close();

    Ok(response)
}
