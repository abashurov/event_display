use models::{ShortEvent, ShortEventVote};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortEventListMsg {
    pub shortevents: Vec<ShortEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMsg {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortEventInfoMsg {
    pub info: ShortEvent,
}
