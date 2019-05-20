use futures::future::Future;
use actix_web::http::header::HeaderValue;
use actix_web::middleware::session::RequestSession;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, State};

use crate::routes::AppState;
use crate::utils::tokens::{token_from_claims, Credentials};
use crate::database::users::messages::{GetUserInfo, GetUserPassword, SetUserPassword};

#[derive(Deserialize, Serialize)]
pub struct Password {
    password: String,
}

pub fn login(
    (credentials, state): (Json<Credentials>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetUserPassword {
            adlogin: credentials.login.clone(),
        })
        .from_err()
        .and_then(move |real_password| {
            if real_password.is_err() {
                return Ok(HttpResponse::Forbidden().into());
            }
            let real_password_value = real_password.unwrap().password;
            if let Ok(check) = bcrypt::verify(credentials.password.clone(), &real_password_value) {
                if !check {
                    return Ok(HttpResponse::Forbidden().into());
                } else {
                    match token_from_claims(credentials.login.clone()) {
                        Ok(token) => match HeaderValue::from_str(token.as_str()) {
                            Ok(token_header) => Ok(HttpResponse::Ok()
                                .header("X-Auth-Token", token_header)
                                .finish()),
                            Err(e) => {
                                warn!("Token generation failed: {}", e);
                                Ok(HttpResponse::InternalServerError().into())
                            }
                        },
                        Err(e) => {
                            warn!("Token generation failed: {}", e);
                            Ok(HttpResponse::InternalServerError().into())
                        }
                    }
                }
            } else {
                return Ok(HttpResponse::Forbidden().into());
            }
        })
        .responder()
}

pub fn get_active_user(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    if let Ok(session_data) = req.session().get::<String>("adlogin") {
        match session_data {
            Some(adlogin) => {
                let fut = req.state().db.send(GetUserInfo {
                    adlogin: adlogin.clone(),
                })
                    .map_err(actix_web::error::ErrorInternalServerError)
                    .and_then(| user | {
                        match user {
                            Ok(user_data) => {
                                return Ok(HttpResponse::Ok().json(user_data.result))
                            },
                            Err(e) => {
                                warn!("Registered inconsistency, auth extracted token, but the user does not exist, or connection failed; DB said: {}", e);
                                return Err(actix_web::error::ErrorInternalServerError("Oops. Something is wrong"));
                            }
                        }
                    });
                return Box::new(fut);
            }
            None => {
                info!("Logged attempt to access secure route without prior authentication");
                return Box::new(futures::future::err(actix_web::error::ErrorUnauthorized(
                    "Access denied",
                )));
            }
        }
    } else {
        warn!("Failed to extract user data from session");
        return Box::new(futures::future::err(actix_web::error::ErrorUnauthorized(
            "Access denied",
        )));
    }
}

pub fn update_password(
    (new_password, req): (Json<Password>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    if let Ok(session_data) = req.session().get::<String>("adlogin") {
        if let Some(adlogin) = session_data {
            req.state()
                .db
                .send(SetUserPassword {
                    adlogin: adlogin.clone(),
                    password: new_password.password.clone(),
                })
                .from_err()
                .and_then(move |status| match status {
                    Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
                    Err(e) => {
                        warn!("Failed to update password: {}", e);
                        Ok(HttpResponse::InternalServerError().into())
                    }
                })
                .responder()
        } else {
            return Box::new(futures::future::err(actix_web::error::ErrorUnauthorized(
                "Access denied",
            )));
        }
    } else {
        return Box::new(futures::future::err(actix_web::error::ErrorUnauthorized(
            "Access denied",
        )));
    }
}
