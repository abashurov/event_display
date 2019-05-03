use actix_web::{App, HttpRequest, HttpResponse, http, error, State, Json, AsyncResponder, FutureResponse};
use jwt::{encode, decode, Header, Algorithm, Validation};
use std::time::SystemTime;
use std::error::Error;
use chrono::Local;

use futures::future::Future;

use crate::database::users::messages::{GetUserPassword};
use crate::app::AppState;

const PREFIX: &str = "Bearer ";
const SECOF30DAYS: u64 = 2592000;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
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






pub fn get_active_user(req: HttpRequest<AppState>) -> Result<Option<User>, AuthError> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        match auth_header.to_str() {
            Ok(header) => {
                if !header.startsWith(PREFIX) {
                    Err(AuthError { message: "Invalid authorization method" })
                }
                let token: String = header.replacen(PREFIX, "", 1);
                user_from_auth(token, req.state().secret, req.state().db)
            },
            Err(e) => {
                Err(AuthError { message: format!("Could not extract token: {}", e) } )
            }
        }
    } else {
        None
    }
}

fn user_from_auth(header: &String, secret: &String, connection: &PgConnection) -> Result<User, AuthError> {
    match decode(header, secret, &Validation::new(Algorithm::HS256)) {
        Ok(claim) => {
            users::find(claim.sub)
        }, 
        Err(e) => {
            Err(AuthError { message: e })
        }
    }
}



struct ListGroups {}

impl Message for ListGroups {
    type Result = Result<Vec<EventGroup>, Error>;
}

impl Handler<ListGroups> for DbExecutor {
    type Result = Result<Vec<EventGroup>, Error>;

    fn handle(&mut self, msg: ListGroups, _: &mut Self::Context) -> Self::Result {
        groups::list()
    }
}
*/

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    login: String,
    password: String,
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