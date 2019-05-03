use actix_web::{Error, actix::Message};

use responses::GroupListMsg;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetGroupList {}

impl Message for GetGroupList {
    type Result = Result<GroupListMsg, Error>;
}