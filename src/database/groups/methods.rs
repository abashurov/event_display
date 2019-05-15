use super::models::EventGroup;
use crate::database::schema::event_groups::dsl::*;

use diesel::prelude::*;

pub fn list(connection: &PgConnection) -> Result<Vec<EventGroup>, diesel::result::Error> {
    event_groups.load::<EventGroup>(connection)
}
