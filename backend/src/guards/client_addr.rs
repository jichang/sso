use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::net::IpAddr;

pub struct ClientAddr(pub IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for ClientAddr {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ClientAddr, ()> {
        match request.client_ip() {
            Some(ip_addr) => Outcome::Success(ClientAddr(ip_addr)),
            None => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}
