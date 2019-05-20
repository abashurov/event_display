use actix_web::http::header::HeaderValue;
use actix_web::middleware::session::RequestSession;
use actix_web::middleware::{Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse};

use crate::routes::AppState;
use crate::utils::tokens::{claims_from_token, token_from_claims, Claims};

const PREFIX: &str = "Bearer ";

pub struct AuthMiddleware;

impl Middleware<AppState> for AuthMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        /* We look for the Authorization header with the "Bearer <token>" contents */
        if let Some(header) = req.headers().get("Authorization") {
            match header.to_str() {
                Ok(token) => {
                    if let Some(claims) = verify_token(token.to_string().replacen(PREFIX, "", 1)) {
                        match req.session().set("adlogin", claims.sub) {
                            Ok(_) => return Ok(Started::Done),
                            Err(e) => {
                                warn!("Dropping request: could not save info to session: {}", e);
                                return Err(actix_web::error::ErrorInternalServerError(
                                    "Authentication failed",
                                ));
                            }
                        }
                    }
                    return Err(actix_web::error::ErrorUnauthorized("Access denied"));
                }
                Err(e) => {
                    debug!("Failed to parse header: {}", e);
                    return Err(actix_web::error::ErrorUnauthorized("Access denied"));
                }
            }
        }
        Err(actix_web::error::ErrorUnauthorized("Access denied"))
    }

    fn response(
        &self,
        req: &HttpRequest<AppState>,
        mut res: HttpResponse,
    ) -> actix_web::Result<Response> {
        /* Reauthenticate users so the token does not get close to the expiration date */
        match req.session().get::<String>("adlogin") {
            Ok(adlogin) => {
                if let Some(login) = adlogin {
                    match token_from_claims(login.clone()) {
                        Ok(token) => {
                            if let Ok(token_header) = HeaderValue::from_str(token.as_str()) {
                                res.headers_mut().append("X-Auth-Token", token_header);
                            }
                            return Ok(Response::Done(res));
                        }
                        Err(e) => {
                            warn!("Failed to generate new token: {}", e);
                            return Ok(Response::Done(res));
                        }
                    }
                }
                warn!("Failed to extract username from the session");
                return Ok(Response::Done(res));
            }
            Err(e) => {
                warn!("Failed to update user's token: {}", e);
                return Ok(Response::Done(res));
            }
        }
    }
}

fn verify_token(token: String) -> Option<Claims> {
    if let Ok(claims) = claims_from_token(token) {
        return Some(claims);
    }
    None
}
