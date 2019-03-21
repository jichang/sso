#![feature(custom_attribute)]
#![feature(proc_macro_hygiene, decl_macro)]

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
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate base32;
extern crate mime;
extern crate oath;
extern crate qrcodegen;
extern crate serde_json;
extern crate serde_repr;
extern crate time;
extern crate toml;
extern crate url;
extern crate uuid;

pub mod common;
pub mod config_parser;
pub mod fairings;
pub mod guards;
pub mod handlers;
pub mod models;
pub mod storage;

use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use r2d2_redis::RedisConnectionManager;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::templates::Template;
use storage::{Cache, Database};

pub fn create(config_file: &str) -> Rocket {
    let config = config_parser::parse(config_file);

    let postgres_manager =
        PostgresConnectionManager::new(config.postgres.addr.as_str(), TlsMode::None)
            .expect("failed to initialize postgres connection manager");
    let database = Database::new(postgres_manager).expect("failed to create database");

    let redis_manager = RedisConnectionManager::new(config.redis.addr.as_str())
        .expect("failed to initialize redis connection manager");
    let cache = Cache::new(redis_manager).expect("failed to create cache");

    rocket::ignite()
        .attach(AdHoc::on_response(
            "RateLimit",
            fairings::ratelimit::on_response,
        ))
        .attach(Template::fairing())
        .mount(
            "/api/v1/",
            routes![
                handlers::index,
                handlers::role::select_roles,
                handlers::role::create_permission,
                handlers::role::remove_permission,
                handlers::permission::select_permissions,
                handlers::user::signup,
                handlers::user::signin,
                handlers::user::signout,
                handlers::user::change_password,
                handlers::user::select_users,
                handlers::user::verify_totp,
                handlers::totp::select_qrcode,
                handlers::totp::update_totp,
                handlers::group::select_groups,
                handlers::group::select_users,
                handlers::summary::select_summary,
                handlers::preference::select_preferences,
                handlers::preference::create_preference,
                handlers::contact::select_types,
                handlers::contact::create_contact,
                handlers::contact::select_contacts,
                handlers::contact::remove_contact,
                handlers::token::create_token,
                handlers::token::delete_token,
                handlers::gender::select_genders,
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
