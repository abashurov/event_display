use actix_web::{actix::Message, Error};

use super::models::{Event, EventAssignee};
use super::responses::{EventAssigneeListMsg, EventListMsg, StatusMsg};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetGroupEvents {
    pub group_id: i32,
}

impl Message for GetGroupEvents {
    type Result = Result<EventListMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddGroupEvent {
    pub event: Event,
}

impl Message for AddGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteGroupEvent {
    pub event_id: i32,
}

impl Message for DeleteGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignGroupEvent {
    pub event_id: i32,
    pub user_name: String,
}

impl Message for AssignGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeassignGroupEvent {
    pub event_id: i32,
    pub user_name: String,
}

impl Message for DeassignGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetGroupEvent {
    pub event: Event,
}

impl Message for SetGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetGroupEventAssigneeList {
    pub event_id: i32,
}

impl Message for GetGroupEventAssigneeList {
    type Result = Result<EventAssigneeListMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserGroupEventList {
    pub user_name: String,
}

impl Message for GetUserGroupEventList {
    type Result = Result<EventAssigneeListMsg, Error>;
}
