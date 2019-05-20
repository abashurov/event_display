use actix_web::{actix::Handler, error, Error};

use super::models::*;
use super::methods::*;
use super::messages::*;
use super::responses::{ShortEventListMsg, StatusMsg};

use crate::database::DbExec;

impl Handler<ListShortEvents> for DbExec {
    type Result = Result<ShortEventListMsg, Error>;

    fn handle(&mut self, _: ListShortEvents, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list(db_conn) {
            Ok(event_list) => Ok(ShortEventListMsg {
                result: event_list,
            }),
            Err(e) => {
                warn!("Query for the event list failed: {}", e);
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<GetShortEvent> for DbExec {
    type Result = Result<ShortEventListMsg, Error>;

    fn handle(&mut self, message: GetShortEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match get(db_conn, message.shortevent_id) {
            Ok(event_info) => Ok(ShortEventListMsg {
                result: event_info,
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<AddShortEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: AddShortEvent, _: &mut Self::Context) -> Self::Result {
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

impl Handler<DeleteShortEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: DeleteShortEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        if let Ok(shortevent_meta) = get(db_conn, message.shortevent_id) {
            if let Some(shortevent) = shortevent_meta.get(0) {
                if shortevent.0.user_name != message.user_name {
                    Err(error::ErrorForbidden("You are not the owner of the event, please vote instead"))
                } else {
                    match delete(db_conn, message.shortevent_id) {
                        Ok(_) => Ok(StatusMsg {
                            status: 0,
                            message: String::from("New event incoming"),
                        }),
                        Err(e) => Err(error::ErrorInternalServerError(e)),
                    }
                }
            } else {
                Err(error::ErrorNotFound("Unable to find event"))
            }
        } else {
            Err(error::ErrorUnauthorized("Are you authenticated?"))
        }
    }
}

impl Handler<VoteShortEvent> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, message: VoteShortEvent, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match register_vote(db_conn, message.vote) {
            Ok(_) => {
                if let Err(e) = check_votes(db_conn) {
                    warn!("Failed to check short event votes: {}", e);
                };
                if let Err(e) = cleanup(db_conn) {
                    warn!("Failed to clean up inactive short event votes: {}", e);
                };
                Ok(StatusMsg {
                    status: 0,
                    message: String::from("Vote applied"),
                })
            }
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}
