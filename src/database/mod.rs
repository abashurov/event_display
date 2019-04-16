mod schema;

pub mod models;
pub mod methods;

use schema::short_event_votes::dsl::*;
use schema::event_assignees::dsl::*;
use schema::display_tokens::dsl::*;
use schema::short_events::dsl::*;
use schema::event_groups::dsl::*;
use schema::events::dsl::*;
use schema::users::dsl::*;
use models::*;

use diesel::prelude::*;

pub fn get_connection(connection_string: &str) -> Result<PgConnection, ConnectionError> {
  PgConnection::establish(connection_string)
}



pub fn get_user_password(connection: &PgConnection, login: &String) -> Result<String, diesel::result::Error> {
  users.select((
    password
  ))
  .filter(users.adlogin.eq(login))
  .first::<String>(connection)
}

pub fn find_display_token(connection: &PgConnection, check_token: &String) -> Result<bool, diesel::result::Error> {
  display_tokens.select(
    exists(
      display_tokens.filter(
        token.eq(check_token)
      )
    )
  )
  .get_result(connection)
}

pub fn 