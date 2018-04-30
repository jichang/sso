#![feature(plugin, custom_derive)]
#![feature(decl_macro)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate argon2rs;
extern crate chrono;
extern crate hex;
extern crate jsonwebtoken as jwt;
extern crate lettre;
extern crate lettre_email;
extern crate md5;
#[macro_use]
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_redis;
extern crate rand;
extern crate redis;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate mime;
extern crate serde_json;
extern crate toml;
extern crate url;
extern crate uuid;

pub mod common;
pub mod config;
pub mod fairings;
pub mod guards;
pub mod handlers;
pub mod models;
pub mod storage;

use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2_redis::RedisConnectionManager;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::Template;
use storage::{Cache, Database};

pub fn create() -> Rocket {
    let config = config::parse();

    let postgres_manager =
        PostgresConnectionManager::new(config.postgres.addr.as_str(), TlsMode::None)
            .expect("failed to initialize postgres connection manager");
    let database = Database::new(postgres_manager).expect("failed to create database");

    let redis_manager = RedisConnectionManager::new(config.redis.addr.as_str())
        .expect("failed to initialize redis connection manager");
    let cache = Cache::new(redis_manager).expect("failed to create cache");

    rocket::ignite()
        .attach(AdHoc::on_response(fairings::ratelimit::on_response))
        .attach(Template::fairing())
        .mount(
            "/api/v1/",
            routes![
                handlers::index,
                handlers::role::select_roles,
                handlers::user::signup,
                handlers::user::signin,
                handlers::user::signout,
                handlers::contact::select_types,
                handlers::contact::create_contact,
                handlers::contact::select_contacts,
                handlers::contact::verify_contact,
                handlers::contact::remove_contact,
                handlers::contact::send_verify_token,
                handlers::profile::select_genders,
                handlers::profile::create_profile,
                handlers::profile::select_profile,
                handlers::profile::remove_profile,
                handlers::application::create_application,
                handlers::application::select_applications,
                handlers::application::select_application,
                handlers::application::remove_application,
                handlers::application::create_scope,
                handlers::application::select_scopes,
                handlers::application::remove_scope,
                handlers::authorization::create_authorization,
                handlers::authorization::select_authorizations,
                handlers::authorization::remove_authorization,
                handlers::authorization::preview_authorization,
                handlers::ticket::create_ticket,
                handlers::ticket::update_ticket,
            ],
        )
        .manage(config)
        .manage(database)
        .manage(cache)
}
