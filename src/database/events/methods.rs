use super::models::{Event, EventAssignee, InsertableEvent};
use crate::database::schema::event_assignees::dsl::*;
//use crate::database::schema::event_groups::dsl::*;
use crate::database::schema::events::dsl::*;

use diesel::prelude::*;

pub fn get(
    connection: &PgConnection,
    target_event_id: i32,
) -> Result<Vec<(Event, EventAssignee)>, diesel::result::Error> {
    events
        .find(target_event_id)
        .inner_join(event_assignees)
        .load::<(Event, EventAssignee)>(connection)
}

pub fn list(
    connection: &PgConnection,
    target_group_id: i32,
) -> Result<Vec<Event>, diesel::result::Error> {
    events
        .filter(group_id.eq(target_group_id))
        .load::<Event>(connection)
}

pub fn list_by_day(
    connection: &PgConnection,
    target_group_id: i32,
    target_day: i16,
) -> Result<Vec<Event>, diesel::result::Error> {
    events
        .filter(group_id.eq(target_group_id).and(day.eq(target_day)))
        .load::<Event>(connection)
}

/*
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
*/

pub fn insert(
    connection: &PgConnection,
    event: &InsertableEvent,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(events)
        .values(event)
        .execute(connection)
}

pub fn delete(
    connection: &PgConnection,
    target_event_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(events)
        .filter(id.eq(target_event_id))
        .execute(connection)
}

pub fn assign(
    connection: &PgConnection,
    target_event_id: i32,
    target_user_name: String,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(event_assignees)
        .values(EventAssignee {
            event_id: target_event_id,
            user_name: target_user_name,
        })
        .execute(connection)
}

pub fn deassign(
    connection: &PgConnection,
    target_event_id: i32,
    target_user_name: String,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(event_assignees)
        .filter(
            event_id
                .eq(target_event_id)
                .and(user_name.eq(target_user_name)),
        )
        .execute(connection)
}

pub fn list_assignees(
    connection: &PgConnection,
    target_event_id: i32,
) -> Result<Vec<(Event, EventAssignee)>, diesel::result::Error> {
    events
        .inner_join(event_assignees)
        .filter(id.eq(target_event_id))
        .load::<(Event, EventAssignee)>(connection)
}

pub fn list_events(
    connection: &PgConnection,
    target_user_name: String,
) -> Result<Vec<(Event, EventAssignee)>, diesel::result::Error> {
    events
        .inner_join(event_assignees)
        .filter(user_name.eq(target_user_name))
        .load::<(Event, EventAssignee)>(connection)
}
