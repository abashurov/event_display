use super::models::EventGroup;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupListMsg {
    pub result: Vec<EventGroup>,
}
