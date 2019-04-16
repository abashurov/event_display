use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct DisplayToken {
  pub id: i32,
  pub token: String,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct EventAssignee {
  pub userId: i32,
  pub eventId: i32,
}

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct EventGroup {
  pub id: i32,
  pub name: String,
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

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct User {
  pub id: i32,
  pub adlogin: String,
  pub displayName: String,
  pub absent: bool,
  pub password: String,
  pub superuser: bool,
  pub availability: i8,
}

#[derive(Debug, Queryable)]
pub struct ExposableUser {
  pub adlogin: String,
  pub displayName: String,
  pub absent: bool,
  pub superuser: bool,
  pub availability: i8,
}