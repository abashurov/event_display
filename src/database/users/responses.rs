use super::models::ExposableUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMsg {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListMsg {
    pub result: Vec<ExposableUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoMsg {
    pub result: ExposableUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPasswordMsg {
    pub password: String,
}
