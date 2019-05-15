use crate::database::schema::event_groups;

#[derive(Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize)]
pub struct EventGroup {
    pub id: i32,
    pub display_name: String,
}
