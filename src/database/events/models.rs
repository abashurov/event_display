use chrono::NaiveDateTime;

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct EventAssignee {
    pub userId: i32,
    pub eventId: i32,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct Event {
    pub id: i32,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub day: i8,
    pub eventType: i8,
    pub groupId: i32,
}
