use super::models::{ExposableUser, UpdateableUser, User};
use crate::database::schema::users::dsl::*;

use diesel::prelude::*;

pub fn find(
    connection: &PgConnection,
    target_adlogin: &String,
) -> Result<ExposableUser, diesel::result::Error> {
    users
        .filter(adlogin.eq(target_adlogin))
        .select((adlogin, display_name, absent, superuser, availability))
        .first::<ExposableUser>(connection)
}

pub fn list(connection: &PgConnection) -> Result<Vec<ExposableUser>, diesel::result::Error> {
    users
        .select((adlogin, display_name, absent, superuser, availability))
        .load::<ExposableUser>(connection)
}

pub fn update_partial(
    connection: &PgConnection,
    user: &UpdateableUser,
) -> Result<User, diesel::result::Error> {
    user.save_changes(connection)
}

pub fn update_password(
    connection: &PgConnection,
    target_adlogin: &String,
    new_password: &String,
) -> Result<usize, diesel::result::Error> {
    diesel::update(users)
        .filter(adlogin.eq(target_adlogin))
        .set(password.eq(new_password))
        .execute(connection)
}

pub fn get_password(
    connection: &PgConnection,
    target_adlogin: &String,
) -> Result<String, diesel::result::Error> {
    users
        .select(password)
        .filter(adlogin.eq(target_adlogin))
        .first::<String>(connection)
}

pub fn add(connection: &PgConnection, user: &User) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(users).values(user).execute(connection)
}

pub fn delete(
    connection: &PgConnection,
    target_adlogin: &String,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(users)
        .filter(adlogin.eq(target_adlogin))
        .execute(connection)
}
