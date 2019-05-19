use crate::database::schema::users;

#[derive(AsChangeset, Debug, Clone, Insertable, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(adlogin)]
pub struct User {
    pub adlogin: String,
    pub display_name: String,
    pub absent: bool,
    pub password: String,
    pub role: i16,
    pub availability: i16,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ExposableUser {
    pub adlogin: String,
    pub display_name: String,
    pub absent: bool,
    pub role: i16,
    pub availability: i16,
}

#[derive(AsChangeset, Identifiable, Default, Debug, Serialize, Deserialize)]
#[primary_key(adlogin)]
#[table_name = "users"]
pub struct UpdateableUser {
    pub adlogin: String,
    pub display_name: Option<String>,
    pub absent: Option<bool>,
    pub password: Option<String>,
    pub role: Option<i16>,
    pub availability: Option<i16>,
}
