use actix_web::{App, HttpRequest, HttpResponse, http, error, State, Json, AsyncResponder, FutureResponse};
use jwt::{encode, decode, Header, Algorithm, Validation};
use std::time::SystemTime;
use std::error::Error;
use chrono::Local;

use futures::{finished, {future::Future}};

use crate::database::users::messages::{GetUserPassword, GetUserInfo};
use crate::database::users::models::ExposableUser;
use crate::app::AppState;

const PREFIX: &str = "Bearer ";
const SECOF30DAYS: u64 = 2592000;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    login: String,
    password: String,
}

/*
fn try_user_auth(adlogin: &String, password: &String, secret: &String) -> Result<String, AuthError> {
    const SECOF30DAYS = 2592000;
    if !database::verifyUser(adlogin, string) {
        Err(AuthError { message: "Adlogin/password authentication failed" })
    }
    let claim = Claims {
        sub: adlogin,
        iat: iat: Local::now().timestamp(),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) + SECOF30DAYS,
    };
    encode(&Header::default(), &claim, secret).map_err(| err |
        AuthError { message: err }
    )
}

fn try_display_auth(token: &String, secret: &String) -> Result<String, AuthError> {
    const SECOF365DAYS = 31536000;
    if !database::verifyDisplay(token) {
        Err(AuthError { message: "Token authentication failed" })
    }
    let claim = Claims {
        sub: token,
        iat: Local::now().timestamp(),
        exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) + SECOF365DAYS,
    };
    encode(&Header::default(), &claim, secret).map_err(| err |
        AuthError { message: err }
    )
}





struct AuthUser {
    pub login: String,
    pub password: String,
}

impl Message for AuthError {
    type Result = Result<>;
}


*/



pub fn get_active_user(req: HttpRequest<AppState>) -> Option<ExposableUser> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        match auth_header.to_str() {
            Ok(header) => {
                if !header.starts_with(PREFIX) {
                    return None
                }
                let token: String = header.replacen(PREFIX, "", 1);
                match decode::<Claims>(&token, req.state().secret.as_bytes(), &Validation::new(Algorithm::HS256)) {
                    Ok(claim) => {
                        let user_info = req.state().db.send(GetUserInfo {
                            adlogin: claim.claims.sub,
                        })
                            .flatten()
                            .wait();
                        if let Ok(info) = user_info {
                            Some(info.info)
                        } else {
                            None
                        }
                    }, 
                    Err(_) => {
                        None
                    }
                }
            },
            Err(_) => {
                None
            }
        }
    } else {
        None
    }
}

pub fn get_current(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    if let Some(user) = get_active_user(req) {
        HttpResponse::Ok().json(user)
            .responder()
    }
    HttpResponse::Ok().body("test")
        .responder()
}

pub fn login((credentials, state): (Json<Credentials>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(GetUserPassword {
        adlogin: credentials.login.clone(),
    })
        .from_err()
        .and_then(move | real_password | {
            if real_password.is_err() {
                return Ok(HttpResponse::Forbidden().into())
            }
            let real_password_value = real_password.unwrap().password;
            if let Ok(check) = bcrypt::verify(credentials.password.clone(), &real_password_value) {
                if !check {
                    return Ok(HttpResponse::Forbidden().into())
                } else {
                    let target_date = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let claim = Claims {
                        sub: credentials.login.clone(),
                        iat: Local::now().timestamp() as u64,
                        exp: target_date.as_secs() + SECOF30DAYS,
                    };
                    match encode(&Header::default(), &claim, state.secret.clone().as_bytes()) {
                        Ok(token) => {
                            Ok(HttpResponse::Ok().json(token))
                        },
                        Err(e) => {
                            Err(error::ErrorInternalServerError(e))
                        }
                    }
                }
            } else {
                return Ok(HttpResponse::Forbidden().into())
            }
        }).responder()
}