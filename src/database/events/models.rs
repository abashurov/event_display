use chrono::NaiveTime;

use crate::database::schema::event_assignees;
use crate::database::schema::events;

#[derive(
    Associations, Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize,
)]
#[primary_key(user_name, event_id)]
#[belongs_to(Event, foreign_key = "event_id")]
pub struct EventAssignee {
    pub event_id: i32,
    pub user_name: String,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub time_from: NaiveTime,
    pub time_to: NaiveTime,
    pub day: i16,
    pub event_type: i16,
    pub group_id: i32,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "events"]
pub struct InsertableEvent {
    pub time_from: NaiveTime,
    pub time_to: NaiveTime,
    pub day: i16,
    pub event_type: i16,
    pub group_id: i32,
    pub display_name: String,
}
