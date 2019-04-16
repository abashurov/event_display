use schema::display_tokens::dsl::*;
use schema::users::dsl::*;

use diesel::prelude::*;

pub fn get_password(connection: &PgConnection, login: &String) -> Result<String, diesel::result::Error> {
  users.select((
    password
  ))
  .filter(users.adlogin.eq(login))
  .first::<String>(connection)
}

pub fn token_exists(connection: &PgConnection, check_token: &String) -> Result<bool, diesel::result::Error> {
  display_tokens.select(
    exists(
      display_tokens.filter(
        token.eq(check_token)
      )
    )
  )
  .get_result(connection)
}