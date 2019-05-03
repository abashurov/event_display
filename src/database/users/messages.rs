use actix_web::{Error, actix::Message};

use super::responses::{StatusMsg, UserListMsg, UserInfoMsg, UserPasswordMsg};
use super::models::User;

#[derive(Deserialize, Serialize, Debug)]
pub struct ListUsers {}

impl Message for ListUsers {
    type Result = Result<UserListMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserInfo {
    pub adlogin: String,
}

impl Message for GetUserInfo {
    type Result = Result<UserInfoMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetUserInfo {
    pub user: User,
}

impl Message for SetUserInfo {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetUserPassword {
    pub adlogin: String,
    pub password: String,
}

impl Message for SetUserPassword {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserPassword {
    pub adlogin: String,
}

impl Message for GetUserPassword {
    type Result = Result<UserPasswordMsg, Error>;
}

/*
#[derive(Deserialize, Serialize, Debug)]
pub struct AddUser {
    pub adlogin: String,
    pub displayName: String,
    pub absent: bool,
    pub password: String,
    pub superuser: bool,
    pub availability: i8,
}

impl Message for AddUser {
    type Result = Result<StatusMsg, Error>;
}
*/