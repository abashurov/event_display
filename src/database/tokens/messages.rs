use actix_web::{Error, actix::Message};

use models::DisplayToken;
use responses::TokenInfoMsg;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTokenInfo {
    pub token: String,
}

impl Message for GetTokenInfo {
    type Result = Result<TokenInfoMsg, Error>;
}