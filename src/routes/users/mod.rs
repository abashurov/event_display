use futures::future::Future;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Json, Path, State};

use crate::routes::AppState;
use crate::database::users::messages::*;
use crate::database::users::models::{ExposableUser, UpdateableUser, User};

#[derive(Serialize, Deserialize)]
pub struct UserChangeSet {
    pub display_name: Option<String>,
    pub absent: Option<bool>,
    pub password: Option<String>,
    pub role: Option<i16>,
    pub availability: Option<i16>,
}

pub fn list_users(state: (State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(ListUsers {})
        .from_err()
        .and_then(move |userlist| match userlist {
            Ok(userlist_values) => Ok(HttpResponse::Ok().json(userlist_values)),
            Err(e) => {
                warn!("Failed to list users: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn get_user((adlogin, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetUserInfo {
            adlogin: adlogin.to_string(),
        })
        .from_err()
        .and_then(move |user| match user {
            Ok(user_info) => Ok(HttpResponse::Ok().json(user_info)),
            Err(e) => {
                warn!("Failed to create a new user: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn add_user((user, state): (Json<User>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(AddUser {
            new_user: user.into_inner(),
        })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to create a new user: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn delete_user(
    (adlogin, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(DeleteUser {
            adlogin: adlogin.to_string(),
        })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to delete a user: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

pub fn update_user(
    (user, state, adlogin): (Json<UserChangeSet>, State<AppState>, Path<String>),
) -> FutureResponse<HttpResponse> {
    let mut user_update: UpdateableUser = UpdateableUser {
        adlogin: adlogin.to_string(),
        absent: user.absent,
        display_name: user.display_name.clone(),
        password: user.password.clone(),
        role: user.role,
        availability: user.availability,
    };
    if let Some(password) = &user.password {
        match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(hashed_password) => {
                user_update.password = Some(hashed_password);
            }
            Err(e) => {
                warn!("Failed to generate hashed password: {}", e);
                return Box::new(futures::future::ok(
                    HttpResponse::InternalServerError().into(),
                ));
            }
        }
    }
    state
        .db
        .send(SetUserInfo { user: user_update })
        .from_err()
        .and_then(move |status| match status {
            Ok(status_info) => Ok(HttpResponse::Ok().json(status_info)),
            Err(e) => {
                warn!("Failed to update user: {}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}
