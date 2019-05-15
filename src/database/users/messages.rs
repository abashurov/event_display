use actix_web::{actix::Message, Error};

use super::models::{UpdateableUser, User};
use super::responses::{StatusMsg, UserInfoMsg, UserListMsg, UserPasswordMsg};

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
    pub user: UpdateableUser,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct AddUser {
    pub new_user: User,
}

impl Message for AddUser {
    type Result = Result<StatusMsg, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteUser {
    pub adlogin: String,
}

impl Message for DeleteUser {
    type Result = Result<StatusMsg, Error>;
}
