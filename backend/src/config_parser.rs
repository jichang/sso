use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Redis {
    pub addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Postgres {
    pub addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mailer {
    pub server: String,
    pub username: String,
    pub password: String,
    pub verify_link: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub postgres: Postgres,
    pub redis: Redis,
    pub mailer: Mailer,
    pub jwt: Jwt,
}

pub fn parse(file_path: &str) -> Config {
    let mut file = File::open(file_path).expect("Can not open config file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Can not read config file");

    let config: Config = toml::from_str(content.as_str()).expect("Can not parse config file");

    config
}
