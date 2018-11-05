extern crate backend;
extern crate rocket;

use rocket::http::Status;
use rocket::local::Client;

pub mod common;

#[test]
fn test_index_page() {
    common::setup();

    let index_page = "<!DOCTYPE html>\n<html>\n\n<head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <title>Feblr SSO</title>\n  <style>\n    .text--center {\n      text-align: center;\n    }\n  </style>\n</head>\n\n<body>\n  <p class=\"text--center\">Welcome to Feblr SSO</p>\n</body>\n\n</html>";

    let client = Client::new(backend::create()).expect("valid rocket instance");
    let mut response = client.get("/api/v1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(index_page.into()));
}
