use actix_web::{error::ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum BackendError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),

    #[fail(display = "Resource Not Found: {}", _0)]
    NotFound(String),

    #[fail(display = "Access To Resource Denied: {}", _0)]
    Forbidden(String),

    #[fail(display = "Authorization Required")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            ServiceError::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Authorization Required")
            }
        }
    }
}
