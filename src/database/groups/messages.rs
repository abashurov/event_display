use actix_web::{actix::Message, Error};

use super::responses::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct ListGroups {}

impl Message for ListGroups {
    type Result = Result<GroupListMsg, Error>;
}
