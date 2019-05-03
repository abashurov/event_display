use actix_web::{Error, actix::Message};

use models::Event;
use responses::{StatusMsg, EventListMsg, EventAssigneeListMsg};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetGroupEvents {}

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
    pub id: u8,
}

impl Message for DeleteGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignGroupEvent {
    pub eventId: u8,
    pub userId: u8,
}

impl Message for AssignGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeassignGroupEvent {
    pub eventId: u8,
    pub userId: u8,
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
pub struct GetGroupEventAssigneeList {}

impl Message for GetGroupEventAssigneeList {
    type Result = Result<EventAssigneeListMsg, Error>;
}