use diesel::prelude::*;

use super::models::EventGroup;

use crate::database::schema::event_groups::dsl::*;

pub fn list(connection: &PgConnection) -> Result<Vec<EventGroup>, diesel::result::Error> {
    event_groups.load::<EventGroup>(connection)
}
