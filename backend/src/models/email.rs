use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum EmailError {
    Empty,
    Incomplete,
    MissLocal,
    MissDomain,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmailError::Empty => write!(f, "email address must not be empty"),
            EmailError::Incomplete => write!(f, "email address must have local and domain part"),
            EmailError::MissLocal => write!(f, "email address must have local part"),
            EmailError::MissDomain => write!(f, "email address must have domain part"),
        }
    }
}

impl StdError for EmailError {
    fn description(&self) -> &str {
        match *self {
            EmailError::Empty => "email address must not be empty",
            EmailError::Incomplete => "email address must have local and domain part",
            EmailError::MissLocal => "email address must have local part",
            EmailError::MissDomain => "email address must have domain part",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub addr: String,
}

impl Email {
    pub fn new(addr: &str) -> Result<Email, EmailError> {
        if addr.is_empty() {
            return Err(EmailError::Empty);
        }

        let parts: Vec<&str> = addr.rsplitn(2, '@').collect();
        if parts.len() < 2 {
            return Err(EmailError::Incomplete);
        }

        if parts[0].is_empty() {
            return Err(EmailError::MissLocal);
        }

        if parts[1].is_empty() {
            return Err(EmailError::MissDomain);
        }

        Ok(Email {
            addr: addr.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_address() {
        let empty_addr = "";
        let email = Email::new(empty_addr);
        assert!(email.is_err());
    }

    #[test]
    fn test_incomplete_address() {
        let incomplete_addr = "incomplete_addr";
        let email = Email::new(incomplete_addr);
        assert!(email.is_err());

        let miss_local_addr = "@domain";
        let email = Email::new(miss_local_addr);
        assert!(email.is_err());

        let miss_domain_addr = "local@";
        let email = Email::new(miss_domain_addr);
        assert!(email.is_err());
    }
}
