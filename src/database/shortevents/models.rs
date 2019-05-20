use chrono::NaiveTime;
use diesel::sql_types::Integer;

use crate::database::schema::short_event_votes;
use crate::database::schema::short_events;

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize)]
pub struct ShortEvent {
    pub id: i32,
    pub user_name: String,
    pub description: String,
    pub time_begin: NaiveTime,
    pub active: bool,
}

#[derive(
    Debug, Clone, Insertable, Identifiable, Queryable, Associations, Serialize, Deserialize,
)]
#[primary_key(user_name, event_id)]
#[belongs_to(ShortEvent, foreign_key = "event_id")]
pub struct ShortEventVote {
    pub user_name: String,
    pub event_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "short_events"]
pub struct InsertableShortEvent {
    pub user_name: String,
    pub description: String,
    pub time_begin: NaiveTime,
    pub active: bool,
}

#[derive(QueryableByName)]
pub struct GroupedShortEvent {
    #[sql_type = "Integer"]
    pub event_id: i32,
}
