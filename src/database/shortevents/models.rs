#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct ShortEvent {
    pub id: i32,
    pub userId: i32,
    pub description: String,
    pub timeBegin: NaiveDateTime,
    pub active: bool,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct ShortEventVote {
    pub id: i32,
    pub userId: i32,
    pub eventId: i32,
}
