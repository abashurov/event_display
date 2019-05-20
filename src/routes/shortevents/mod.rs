use chrono::offset::Utc;
use futures::future::Future;
use actix_web::middleware::session::RequestSession;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, Path, State};

use crate::routes::AppState;
use crate::database::shortevents::messages::*;
use crate::database::shortevents::models::{InsertableShortEvent, ShortEventVote};

#[derive(Serialize, Deserialize)]
pub struct SerializableShortEvent {
    pub description: String,
    pub active: bool,
}

pub fn list_shortevents(state: (State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(ListShortEvents {})
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

pub fn add_shortevent(
    (event_info, state, req): (
        Json<SerializableShortEvent>,
        State<AppState>,
        HttpRequest<AppState>,
    ),
) -> FutureResponse<HttpResponse> {
    if let Ok(session_data) = req.session().get::<String>("adlogin") {
        match session_data {
            Some(adlogin) => {
                let event: InsertableShortEvent = InsertableShortEvent {
                    user_name: adlogin.clone(),
                    description: event_info.description.clone(),
                    time_begin: Utc::now().time(),
                    active: event_info.active,
                };
                state
                    .db
                    .send(AddShortEvent { event: event })
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
            None => Box::new(futures::future::ok(
                HttpResponse::InternalServerError().into(),
            )),
        }
    } else {
        Box::new(futures::future::ok(
            HttpResponse::InternalServerError().into(),
        ))
    }
}

pub fn get_shortevent(
    (shortevent_id, state): (Path<i32>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetShortEvent {
            shortevent_id: shortevent_id.into_inner(),
        })
        .from_err()
        .and_then(move |shortevent| match shortevent {
            Ok(shortevent) => Ok(HttpResponse::Ok().json(shortevent)),
            Err(e) => {
                warn!("Failed to get shortevent: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn delete_shortevent(
    (shortevent_id, state, req): (Path<i32>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    if let Ok(session_data) = req.session().get::<String>("adlogin") {
        match session_data {
            Some(adlogin) => state
                .db
                .send(DeleteShortEvent {
                    shortevent_id: shortevent_id.into_inner(),
                    user_name: adlogin,
                })
                .from_err()
                .and_then(move |status| match status {
                    Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
                    Err(e) => {
                        warn!("Failed to delete event: {}", e);
                        Ok(HttpResponse::InternalServerError().into())
                    }
                })
                .responder(),
            None => Box::new(futures::future::ok(
                HttpResponse::InternalServerError().into(),
            )),
        }
    } else {
        Box::new(futures::future::ok(
            HttpResponse::InternalServerError().into(),
        ))
    }
}

pub fn register_shortevent_vote(
    (shortevent_id, state, req): (Path<i32>, State<AppState>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    if let Ok(session_data) = req.session().get::<String>("adlogin") {
        match session_data {
            Some(adlogin) => {
                let vote = ShortEventVote {
                    event_id: shortevent_id.into_inner(),
                    user_name: adlogin,
                };
                state
                    .db
                    .send(VoteShortEvent { vote: vote })
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
            None => Box::new(futures::future::ok(
                HttpResponse::InternalServerError().into(),
            )),
        }
    } else {
        Box::new(futures::future::ok(
            HttpResponse::InternalServerError().into(),
        ))
    }
}
