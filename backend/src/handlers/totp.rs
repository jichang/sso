use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use oath::{totp_raw_now, HashType};
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::State;
use rocket_contrib::json::Json;

use super::super::common;
use super::super::config_parser::Config;
use super::super::guards::bearer;
use super::super::guards::bearer::Claims;
use super::super::models::totp;
use super::super::models::totp::Totp;
use super::super::storage::Database;
use super::Error;

use base32::{encode, Alphabet};
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

const TOTP_SECRET_LEN: usize = 12;

#[derive(Serialize, Deserialize)]
pub struct QrCodeConfig {
    size: i32,
    modules: Vec<Vec<bool>>,
}

#[get("/users/<user_id>/totp/qrcode")]
pub fn select_qrcode(
    config: State<Config>,
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    mut cookies: Cookies,
) -> Result<Json<QrCodeConfig>, Error> {
    if claims.uid == user_id {
        let random_bytes = common::gen_rand_bytes(TOTP_SECRET_LEN)?;
        let totp_secret = base32::encode(Alphabet::RFC4648 { padding: true }, &random_bytes);
        let data = format!(
            "otpauth://totp/{}:{}?secret={}&issuer=Feblr",
            config.server.domain,
            user_id,
            totp_secret
        );

        let qr = QrCode::encode_text(&data, QrCodeEcc::Medium).unwrap();
        let size = qr.size();
        let mut modules = vec![];
        for x in 0..size {
            let mut row = vec![];
            for y in 0..size {
                row.push(qr.get_module(x, y));
            }
            modules.push(row);
        }

        let mut cookie = Cookie::new("user.totp.secret", totp_secret);
        cookie.set_http_only(true);
        cookies.add_private(cookie);

        Ok(Json(QrCodeConfig {
            size: size,
            modules: modules,
        }))
    } else {
        Err(Error::Privilege)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TotpCreateParams {
    code: u64,
}

#[post("/users/<user_id>/totp", data = "<params>")]
pub fn update_totp(
    db: State<Database>,
    user_id: i64,
    claims: Claims,
    mut cookies: Cookies,
    params: Json<TotpCreateParams>,
) -> Result<Json<Totp>, Error> {
    if claims.uid == user_id {
        match cookies.get_private("user.totp.secret") {
            Some(totp_secret_cookie) => {
                let conn = db.get_conn()?;
                match base32::decode(
                    Alphabet::RFC4648 { padding: true },
                    totp_secret_cookie.value(),
                ) {
                    Some(totp_secret) => {
                        let code = totp_raw_now(&totp_secret, 6, 0, 30, &HashType::SHA1);
                        if code == params.code {
                            let new_totp = totp::create(&*conn, user_id, &totp_secret)?;
                            cookies.remove_private(Cookie::named("user.totp.secret"));

                            Ok(Json(new_totp))
                        } else {
                            Err(Error::Params)
                        }
                    }
                    None => Err(Error::Params),
                }
            }
            None => Err(Error::Params),
        }
    } else {
        Err(Error::Privilege)
    }
}
