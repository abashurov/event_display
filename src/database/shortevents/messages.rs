use actix_web::{actix::Message, Error};

use super::models::{InsertableShortEvent, ShortEventVote};
use super::responses::{ShortEventListMsg, StatusMsg};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListShortEvents {}

impl Message for ListShortEvents {
    type Result = Result<ShortEventListMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddShortEvent {
    pub event: InsertableShortEvent,
}

impl Message for AddShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteShortEvent {
    pub shortevent_id: i32,
    pub user_name: String,
}

impl Message for DeleteShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VoteShortEvent {
    pub vote: ShortEventVote,
}

impl Message for VoteShortEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetShortEvent {
    pub shortevent_id: i32,
}

impl Message for GetShortEvent {
    type Result = Result<ShortEventListMsg, Error>;
}
