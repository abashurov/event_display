use actix_web::error::{Error, Result};
use actix_web::middleware::session::{SessionBackend, SessionImpl};
use actix_web::middleware::Response;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::{ok as FutOk, FutureResult};
use std::collections::HashMap;

/* We cannot have a session without a backend as of 0.7, so this is a simple backend that only stores session state */
/* TODO: implement FromRequest to replace AuthMiddleware */

pub struct AuthSession {
    update: bool,
    state: HashMap<String, String>,
}

impl SessionImpl for AuthSession {
    fn get(&self, key: &str) -> Option<&str> {
        if let Some(s) = self.state.get(key) {
            Some(s)
        } else {
            None
        }
    }

    fn set(&mut self, key: &str, value: String) {
        self.update = true;
        self.state.insert(key.to_owned(), value);
    }

    fn remove(&mut self, key: &str) {
        self.update = true;
        self.state.remove(key);
    }

    fn clear(&mut self) {
        self.update = true;
        self.state.clear()
    }

    fn write(&self, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}

pub struct AuthSessionBackend();

impl<S> SessionBackend<S> for AuthSessionBackend {
    type Session = AuthSession;
    type ReadFuture = FutureResult<AuthSession, Error>;

    fn from_request(&self, _: &mut HttpRequest<S>) -> Self::ReadFuture {
        FutOk(AuthSession {
            update: false,
            state: HashMap::new(),
        })
    }
}
