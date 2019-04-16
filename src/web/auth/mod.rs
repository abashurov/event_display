use jwt::{encode, decode, Header, Algorithm, Validation};
use std::time::SystemTime;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

struct AuthError {
  message: String,
}

impl Error for AuthError {
  fn description(&self) -> &str {
    "Generic article processing error"
  }

  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

/* std::env::var("JWT_SECRET")
    .expect("Connection string to source database must be provided via SOURCE_DB environment variable") */

fn tryUserAuth(adlogin: &String, password: &String, secret: &String) -> Result<String, AuthError> {
  const SECOF30DAYS = 2592000;
  if !database::verifyUser(adlogin, string) {
    Err(AuthError { message: "Adlogin/password authentication failed" })
  }
  let claim = Claims {
    sub: adlogin,
    exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) + SECOF30DAYS,
  };
  encode(&Header::default(), &claim, secret).map_err(| err |
    AuthError { message: err }
  )
}

fn tryDisplayAuth(token: &String, secret: &String) -> Result<String, AuthError> {
  const SECOF365DAYS = 31536000;
  if !database::verifyDisplay(token) {
    Err(AuthError { message: "Token authentication failed" })
  }
  let claim = Claims {
    sub: token,
    exp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) + SECOF365DAYS,
  };
  encode(&Header::default(), &claim, secret).map_err(| err |
    AuthError { message: err }
  )
}

fn verifyAuth(header: &String, secret: &String) -> Result<String, AuthError> {
  match decode(header, secret, &Validation::new(Algorithm::HS256)) {
    Ok(claim) => {
      Ok(claim.sub)
    }, 
    Err(e) => {
      Err(AuthError { message: e })
    }
  }
}