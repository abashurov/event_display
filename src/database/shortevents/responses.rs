use super::models::{ShortEvent, ShortEventVote};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortEventListMsg {
    pub result: Vec<(ShortEvent, Vec<ShortEventVote>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMsg {
    pub status: i32,
    pub message: String,
}
