use crate::database::schema::users;

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub adlogin: String,
    #[column_name="display_name"]
    pub displayName: String,
    pub absent: bool,
    pub password: String,
    pub superuser: bool,
    pub availability: i16,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ExposableUser {
    pub adlogin: String,
    pub displayName: String,
    pub absent: bool,
    pub superuser: bool,
    pub availability: i16,
}
