use actix_web::{actix::Handler, error, Error};

use super::messages::*;
use super::methods::*;
use super::responses::{EventAssigneeListMsg, EventListMsg, StatusMsg};

use crate::database::DbExec;

impl Handler<GetGroupEvents> for DbExec {
    type Result = Result<EventListMsg, Error>;

    fn handle(&mut self, message: GetGroupEvents, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list(db_conn, message.group_id) {
            Ok(event_list) => Ok(EventListMsg { events: event_list }),
            Err(e) => {
                warn!("Query for the event list failed: {}", e);
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<GetGroupEventInfo> for DbExec {
    type Result = Result<EventAssigneeListMsg, Error>;

    fn handle(&mut self, message: GetGroupEventInfo, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match get(db_conn, message.event_id) {
            Ok(event_info) => Ok(EventAssigneeListMsg {
                event_assignees: event_info,
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<AddGroupEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: AddGroupEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match insert(db_conn, &message.event) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("New event incoming"),
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<DeleteGroupEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: DeleteGroupEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match delete(db_conn, message.event_id) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("New event incoming"),
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<AssignGroupEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: AssignGroupEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match assign(db_conn, message.event_id, message.user_name) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("Event assigned"),
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<DeassignGroupEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: DeassignGroupEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match deassign(db_conn, message.event_id, message.user_name) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("Event deassigned"),
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

/*
#[derive(Deserialize, Serialize, Debug)]
pub struct SetGroupEvent {
    pub event: Event,
}

impl Message for SetGroupEvent {
    type Result = Result<StatusMsg, Error>;
}

Do we actually need changeset?
*/

impl Handler<GetGroupEventAssigneeList> for DbExec {
    type Result = Result<EventAssigneeListMsg, Error>;

    fn handle(
        &mut self,
        message: GetGroupEventAssigneeList,
        _: &mut Self::Context,
    ) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list_assignees(db_conn, message.event_id) {
            Ok(assignee_list) => Ok(EventAssigneeListMsg {
                event_assignees: assignee_list,
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<GetUserGroupEventList> for DbExec {
    type Result = Result<EventAssigneeListMsg, Error>;

    fn handle(&mut self, message: GetUserGroupEventList, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list_events(db_conn, message.user_name) {
            Ok(event_list) => Ok(EventAssigneeListMsg {
                event_assignees: event_list,
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}
