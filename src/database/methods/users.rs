use models::{ExposableUser, User};
use schema::users::dsl::*;

use diesel::prelude::*;

pub fn list(connection: &PgConnection) -> Result<Vec<ExposableUser>, diesel::result::Error> {
  users.select((
    adlogin,
    display_name,
    absent,
    superuser,
    availability
  ))
  .load::<ExposableUser>(connection)
}

pub fn update_full(connection: &PgConnection, user: &User) -> Result<(), diesel::result::Error> {
  users.update(
    users.filter(adlogin.eq(user.adlogin))
  )
  .set((
    display_name.eq(user.displayName),
    absent.eq(user.absent),
    superuser.eq(user.superuser),
    availability.eq(user.availability),
    password.eq(password)
  ))
  .execute(connection)
}

pub fn update_password(connection: &PgConnection, login: &String, new_password: &String) -> Result<(), diesel::result::Error> {
  users.update(
    users.filter(adlogin.eq(user.login))
  )
  .set((
    password.eq(new_password)
  ))
  .execute(connection)
}