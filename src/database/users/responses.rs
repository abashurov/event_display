use super::models::{ExposableUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMsg {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListMsg {
    pub users: Vec<ExposableUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoMsg {
    pub info: ExposableUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPasswordMsg {
    pub password: String,
}