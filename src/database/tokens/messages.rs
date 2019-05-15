use actix_web::{actix::Message, Error};

use models::DisplayToken;
use responses::TokenInfoMsg;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTokenInfo {
    pub token: String,
}

impl Message for GetTokenInfo {
    type Result = Result<TokenInfoMsg, Error>;
}
