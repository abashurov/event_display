use super::schema::event_assignees::dsl::*;
use super::schema::event_groups::dsl::*;
use super::schema::events::dsl::*;
use super::schema::users::dsl::*;
use models::{Event, EventAssignee, EventGroup, User};

use diesel::prelude::*;

pub fn list(
    connection: &PgConnection,
    target_group_id: u32,
) -> Result<Vec<Event>, diesel::result::Error> {
    events
        .filter(group_id.eq(target_group_id))
        .load::<Event>(connection)
}

pub fn update(connection: &PgConnection, event: &Event) -> Result<usize, diesel::result::Error> {
    diesel::update(events)
        .filter(id.eq(event.id))
        .set((
            time_from.eq(event.timeFrom),
            time_to.eq(event.timeTo),
            day.eq(event.day),
            event_type.eq(event.eventType),
            group_id.eq(event.groupId),
        ))
        .execute(connection)
}

pub fn insert(connection: &PgConnection, event: &Event) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(events)
        .values(event)
        .execute(connection)
}

pub fn delete(connection: &PgConnection, eventId: u8) -> Result<usize, diesel::result::Error> {
    diesel::delete(events)
        .filter(id.eq(eventId))
        .execute(connection)
}

pub fn assign(
    connection: &PgConnection,
    eventId: u8,
    userId: u8,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(event_assignees)
        .values(EventAssignee {
            eventId: eventId,
            userId: userId,
        })
        .execute(connection)
}

pub fn deassign(
    connection: &PgConnection,
    eventId: u8,
    userId: u8,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(event_assignees)
        .filter((event_id.eq(eventId), user_id.eq(userId)))
        .execute(connection)
}
