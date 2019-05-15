use actix_web::middleware::session::RequestSession;
use actix_web::middleware::{Middleware, Started};
use actix_web::HttpRequest;
use futures::future::Future;

use crate::database::users::messages::GetUserInfo;
use crate::routes::AppState;

pub struct RoleMiddleware;

impl Middleware<AppState> for RoleMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        //This Middleware _must_ be run after the AuthMiddleware, as it relies on session

        debug!(
            "Checking {:?} access...",
            req.session().get::<String>("adlogin")
        );

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
                                    if user_data.info.superuser {
                                        return futures::future::ok(None);
                                    } else {
                                        info!("Registered unauthenticated attempt to access route as {}", user_data.info.adlogin);
                                        return futures::future::err(actix_web::error::ErrorUnauthorized("Access denied"));
                                    }
                                },
                                Err(e) => {
                                    warn!("Registered inconsistency, auth extracted token, but the user does not exist, or connection failed; DB said: {}", e);
                                    return futures::future::err(actix_web::error::ErrorInternalServerError("Oops. Something is wrong"));
                                }
                            }
                        });
                    Ok(Started::Future(Box::new(fut)))
                }
                None => {
                    info!("Logged attempt to access secure route without prior authentication");
                    return Err(actix_web::error::ErrorUnauthorized("Access denied"));
                }
            }
        } else {
            warn!("Failed to extract user data from session");
            return Err(actix_web::error::ErrorUnauthorized("Access denied"));
        }
    }
}
