use actix_web::middleware::session::RequestSession;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, Path, State};
use chrono::NaiveTime;
use futures::future::Future;

use crate::database::events::messages::*;
use crate::database::events::models::{EventAssignee, InsertableEvent};
use crate::routes::AppState;

#[derive(Serialize, Deserialize)]
pub struct SerializableEvent {
    pub time_from: NaiveTime,
    pub time_to: NaiveTime,
    pub day: i16,
    pub event_type: i16,
    pub display_name: String,
}

pub fn list_events(
    (group_id, state): (Path<i32>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetGroupEvents {
            group_id: group_id.into_inner(),
        })
        .from_err()
        .and_then(move |eventlist| match eventlist {
            Ok(eventlist_values) => Ok(HttpResponse::Ok().json(eventlist_values)),
            Err(e) => {
                warn!("Failed to list events: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn add_event(
    (event_info, state, group_id): (Json<SerializableEvent>, State<AppState>, Path<i32>),
) -> FutureResponse<HttpResponse> {
    let event: InsertableEvent = InsertableEvent {
        time_from: event_info.time_from,
        time_to: event_info.time_to,
        day: event_info.day,
        event_type: event_info.event_type,
        group_id: group_id.into_inner(),
        display_name: event_info.display_name.clone(),
    };
    state
        .db
        .send(AddGroupEvent { event: event })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to add event: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn get_event((event_id, state): (Path<i32>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetGroupEventInfo {
            event_id: event_id.into_inner(),
        })
        .from_err()
        .and_then(move |event| match event {
            Ok(event) => Ok(HttpResponse::Ok().json(event)),
            Err(e) => {
                warn!("Failed to get event: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn delete_event(
    (event_id, state): (Path<i32>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DeleteGroupEvent {
            event_id: event_id.into_inner(),
        })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to delete event: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn list_event_assignees(
    (event_id, state): (Path<i32>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetGroupEventAssigneeList {
            event_id: event_id.into_inner(),
        })
        .from_err()
        .and_then(move |eventlist| match eventlist {
            Ok(eventlist_values) => Ok(HttpResponse::Ok().json(eventlist_values)),
            Err(e) => {
                warn!("Failed to list events: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn list_assignee_events(
    (req, state): (HttpRequest<AppState>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    if let Ok(user_data) = req.session().get::<String>("adlogin") {
        if let Some(user_name) = user_data {
            state
                .db
                .send(GetUserGroupEventList {
                    user_name: user_name.to_string(),
                })
                .from_err()
                .and_then(move |eventlist| match eventlist {
                    Ok(eventlist_values) => Ok(HttpResponse::Ok().json(eventlist_values)),
                    Err(e) => {
                        warn!("Failed to list events: {}", e);
                        Ok(HttpResponse::InternalServerError().into())
                    }
                })
                .responder()
        } else {
            Box::new(futures::future::ok(HttpResponse::Forbidden().finish()))
        }
    } else {
        Box::new(futures::future::ok(HttpResponse::Forbidden().finish()))
    }
}

pub fn add_event_assignee(
    (params, state): (Path<(i32, String)>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(AssignGroupEvent {
            event_id: params.0,
            user_name: params.1.clone(),
        })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to assign event: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn delete_event_assignee(
    (params, state): (Path<(i32, String)>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DeassignGroupEvent {
            event_id: params.0,
            user_name: params.1.clone(),
        })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to delete event: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
