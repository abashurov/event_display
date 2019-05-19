use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, Path, State};

use futures::future::Future;

use crate::database::groups::messages::*;
use crate::routes::AppState;

pub fn list_groups(state: (State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(ListGroups {})
        .from_err()
        .and_then(move |grouplist| match grouplist {
            Ok(grouplist_values) => Ok(HttpResponse::Ok().json(grouplist_values)),
            Err(e) => {
                warn!("Failed to list users: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
