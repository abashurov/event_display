use schema::display_tokens::dsl::*;

use diesel::prelude::*;

pub fn token_exists(
    connection: &PgConnection,
    check_token: &String,
) -> Result<bool, diesel::result::Error> {
    display_tokens
        .select(exists(display_tokens.filter(token.eq(check_token))))
        .get_result(connection)
}
