use super::models::{ExposableUser, User};
use crate::database::schema::users::dsl::*;

use diesel::prelude::*;

pub fn find(
    connection: &PgConnection,
    target_login: &String,
) -> Result<ExposableUser, diesel::result::Error> {
    users
        .filter(adlogin.eq(target_login))
        .select((adlogin, display_name, absent, superuser, availability))
        .first::<ExposableUser>(connection)
}

pub fn list(connection: &PgConnection) -> Result<Vec<ExposableUser>, diesel::result::Error> {
    users
        .select((adlogin, display_name, absent, superuser, availability))
        .load::<ExposableUser>(connection)
}

pub fn update_full(connection: &PgConnection, user: &User) -> Result<usize, diesel::result::Error> {
    diesel::update(users)
        .filter(adlogin.eq(user.adlogin.clone()))
        .set((
            display_name.eq(user.displayName.clone()),
            absent.eq(user.absent),
            superuser.eq(user.superuser),
            availability.eq(user.availability),
            password.eq(password),
        ))
        .execute(connection)
}

pub fn update_password(
    connection: &PgConnection,
    login: &String,
    new_password: &String,
) -> Result<usize, diesel::result::Error> {
    diesel::update(users)
        .filter(adlogin.eq(login))
        .set(password.eq(new_password))
        .execute(connection)
}

pub fn get_password(
    connection: &PgConnection,
    login: &String,
) -> Result<String, diesel::result::Error> {
    users
        .select(password)
        .filter(adlogin.eq(login))
        .first::<String>(connection)
}
