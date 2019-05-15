use super::models::{Event, EventAssignee};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMsg {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventListMsg {
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventAssigneeListMsg {
    pub event_assignees: Vec<(Event, EventAssignee)>,
}
