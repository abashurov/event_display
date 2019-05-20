use std::rc::Rc;
use actix_web::HttpRequest;
use futures::future::Future;
use actix_web::middleware::session::RequestSession;
use actix_web::middleware::{Middleware, Started};

use crate::database::users::messages::GetUserInfo;
use crate::routes::AppState;

const SELF_RO_ACCESS: i16 = 0;
const FULL_RO_ACCESS: i16 = 1;
const FULL_RW_ACCESS: i16 = 2;

const DEFAULT_ACCESS_LEVEL: i16 = FULL_RO_ACCESS;

pub struct RoleMiddleware {
    inner: Rc<RoleMiddlewareInner>,
}

struct RoleMiddlewareInner {
    access_level: i16,
}

impl RoleMiddleware {
    pub fn new() -> RoleMiddleware {
        let inner = RoleMiddlewareInner {
            access_level: DEFAULT_ACCESS_LEVEL,
        };
        RoleMiddleware {
            inner: Rc::new(inner),
        }
    }

    pub fn with_user_or_above() -> RoleMiddleware {
        let inner = RoleMiddlewareInner {
            access_level: SELF_RO_ACCESS,
        };
        RoleMiddleware {
            inner: Rc::new(inner),
        }
    }

    pub fn with_display_or_above() -> RoleMiddleware {
        let inner = RoleMiddlewareInner {
            access_level: FULL_RO_ACCESS,
        };
        RoleMiddleware {
            inner: Rc::new(inner),
        }
    }

    pub fn with_manager_or_above() -> RoleMiddleware {
        let inner = RoleMiddlewareInner {
            access_level: FULL_RW_ACCESS,
        };
        RoleMiddleware {
            inner: Rc::new(inner),
        }
    }
}

impl Middleware<AppState> for RoleMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        //This Middleware _must_ be run after the AuthMiddleware, as it relies on session

        debug!(
            "Checking {:?} access...",
            req.session().get::<String>("adlogin")
        );

        /* If only I knew how to make it more bearable without boxing it twice */
        let expected_access = Box::new(self.inner.access_level);
        if let Ok(session_data) = req.session().get::<String>("adlogin") {
            match session_data {
                Some(adlogin) => {
                    let fut = req.state().db.send(GetUserInfo {
                            adlogin: adlogin,
                        })
                            .map_err(actix_web::error::ErrorInternalServerError)
                            .and_then(| user | {
                                let e_access = Box::new(expected_access);
                                match user {
                                    Ok(user_data) => {
                                        if user_data.result.role >= **e_access {
                                            return futures::future::ok(None);
                                        } else {
                                            info!("Registered unauthenticated attempt to access route as {}", user_data.result.adlogin);
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
