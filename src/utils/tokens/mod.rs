use chrono::Local;
use jwt::errors::Error;
use jwt::{decode, encode, Algorithm, Header, Validation};
use std::time::SystemTime;

const JWT_SECRET: &str = "secret";
const SECOF30DAYS: u64 = 2592000;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenCredentials {
    pub token: String,
}

pub fn token_from_claims(sub: String) -> Result<String, Error> {
    let target_date = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let claims = Claims {
        sub: sub,
        iat: Local::now().timestamp() as u64,
        exp: target_date.as_secs() + SECOF30DAYS,
    };
    encode(&Header::default(), &claims, JWT_SECRET.clone().as_bytes())
}

pub fn claims_from_token(token: String) -> Result<Claims, Error> {
    debug!("Got token {}", token);
    match decode::<Claims>(
        &token,
        JWT_SECRET.clone().as_bytes(),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => {
            debug!("Failed to decode token: {}", e);
            Err(e)
        }
    }
}
