use models::{EventGroup, Event, EventAssignee, User};
use schema::event_assignees::dsl::*;
use schema::event_groups::dsl::*;
use schema::events::dsl::*;
use schema::users::dsl::*;

use diesel::prelude::*;

pub fn list(connection: &PgConnection, target_group_id: u32) -> Result<Vec<Event>, diesel::result::Error> {
  events
    .filter(group_id.eq(target_group_id))
    .load::<Event>(connection)
}

pub fn update(connection: &PgConnection, event: &Event) -> Result<(), diesel::result::Error> {
  diesel::update(events)
    .filter(id.eq(event.id))
    

}


