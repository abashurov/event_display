use actix_web::{Error, actix::Message};

use models::ShortEvent;
use responses::{ShortEventListMsg, StatusMsg, ShortEventInfoMsg};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListShortEvents {}

impl Message for ListShortEvents {
    type Result = Result<ShortEventListMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddShortEvent {
    pub event: ShortEvent,
}

impl Message for AddShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteShortEvent {
    pub id: u8,
}

impl Message for DeleteShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VoteShortEvent {
    pub eventId: u8,
    pub userId: u8,
}

impl Message for VoteShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetShortEvent {
    pub eventId: u8,
}

impl Message for GetShortEvent {
    type Result = Result<ShortEventInfoMsg, Error>;
}
