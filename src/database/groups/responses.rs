use models::{EventGroup};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupListMsg {
    pub users: Vec<EventGroup>,
}
