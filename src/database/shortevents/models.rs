use chrono::NaiveTime;

use crate::database::schema::short_event_votes;
use crate::database::schema::short_events;

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct ShortEvent {
    pub id: i32,
    pub user_name: String,
    pub description: String,
    pub time_begin: NaiveTime,
    pub active: bool,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct ShortEventVote {
    pub id: i32,
    pub user_name: String,
    pub event_id: i32,
}
