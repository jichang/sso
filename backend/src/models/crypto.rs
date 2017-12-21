use argon2rs;
use rand::{thread_rng, Rng};
use std::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;

const SALT_SIZE: usize = 32;
const MIN_SIZE: usize = 4;

#[derive(Debug)]
pub enum Error {
    Salt(IoError),
    MinLen(usize),
    NotMatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Salt(ref _err) => "can not generate salt",
            Error::MinLen(_len) => "password is smaller than minimum length",
            Error::NotMatch => "password does not match",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Salt(ref err) => Some(err),
            Error::MinLen(_len) => None,
            Error::NotMatch => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Salt(err)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plaintext {
    pub value: String,
}

impl Plaintext {
    pub fn new(value: &str) -> Result<Plaintext, Error> {
        if value.len() < MIN_SIZE {
            return Err(Error::MinLen(MIN_SIZE));
        }

        Ok(Plaintext { value: value.to_string() })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ciphertext {
    pub salt: String,
    pub hash: Vec<u8>,
}

pub fn generate(plaintext: &Plaintext) -> Result<Ciphertext, Error> {
    let salt: String = thread_rng().gen_ascii_chars().take(SALT_SIZE).collect();
    let hash = argon2rs::argon2i_simple(&plaintext.value, &salt);

    Ok(Ciphertext {
        salt: salt,
        hash: hash.to_vec(),
    })
}

pub fn compare(plaintext: &Plaintext, salt: String, hash: Vec<u8>) -> Result<Ciphertext, Error> {
    let new_hash = argon2rs::argon2i_simple(&plaintext.value, &salt);

    if hash[..] == new_hash[..] {
        Ok(Ciphertext {
            salt: salt,
            hash: hash,
        })
    } else {
        Err(Error::NotMatch)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_crypt() {
        let plaintext = Plaintext::new("foobar").unwrap();
        let ciphertext = generate(&plaintext).unwrap();
        let new_ciphertext = compare(&plaintext, ciphertext.salt.clone(), ciphertext.hash.clone())
            .unwrap();

        assert_eq!(new_ciphertext.salt[..], ciphertext.salt[..]);
        assert_eq!(new_ciphertext.hash[..], ciphertext.hash[..]);
    }
}
