use super::models::{ShortEvent, ShortEventVote};
use crate::database::schema::short_event_votes::dsl::*;
use crate::database::schema::short_events::dsl::*;

use diesel::prelude::*;

pub fn list(connection: &PgConnection) -> Result<Vec<ShortEvent>, diesel::result::Error> {
    short_events
        .filter(active.eq(true))
        .load::<ShortEvent>(connection)
}

pub fn insert(
    connection: &PgConnection,
    shortevent: &ShortEvent,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(short_events)
        .values(shortevent)
        .execute(connection)
}

pub fn delete(connection: &PgConnection, shortEventId: u8) -> Result<usize, diesel::result::Error> {
    diesel::delete(short_events)
        .filter(id.eq(shortEventId))
        .execute(connection)
}

pub fn cleanup(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(short_events)
        .filter(active.eq(false))
        .execute(connection)
}

pub fn register_vote(
    connection: &PgConnection,
    target_short_event_id: i32,
    target_user_name: String,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(short_event_votes)
        .values((
            event_id.eq(target_short_event_id),
            user_name.eq(target_user_name),
        ))
        .execute(connection)
}

pub fn check_votes(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    /*
     * NOTE: This part potentially can blow up; undocumented and unstable API
     *
     * https://github.com/diesel-rs/diesel/issues/210
     */
    match short_event_votes
        .select(event_id)
        .group_by(event_id)
        .filter(sql("COUNT(*) >= 3"))
        .load::<Vec<(u8)>>(connection)
    {
        Ok(events) => {
            diesel::update(shortevents)
                .filter(event_id.eq(any(events)))
                .set(active.eq(false)) // Probably should use delete instead
                .execute(connection)
        }
        Err(e) => Err(e),
    }
}
