#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable)]
pub struct EventGroup {
    pub id: i32,
    pub name: String,
}
