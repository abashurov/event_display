use models::ShortEvent;
use schema::short_event_votes::dsl::*;
use schema::shortevents::dsl::*;

use diesel::prelude::*;

pub fn list(connection: &PgConnection) -> Result<Vec<ShortEvent>, diesel::result::Error> {
    shortevents
        .filter(active.eq(true))
        .load::<ShortEvent>(connection)
}

pub fn insert(
    connection: &PgConnection,
    shortevent: &ShortEvent,
) -> Result<usize, diesel::result::Error> {
    diesel::insert(shortevents)
        .values(shortevent)
        .execute(connection)
}

pub fn delete(connection: &PgConnection, shortEventId: u8) -> Result<usize, diesel::result::Error> {
    diesel::delete(shortevents)
        .filter(id.eq(shortEventId))
        .execute(connection)
}

pub fn cleanup(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(shortevents)
        .filter(active.eq(false))
        .execute(connection)
}

pub fn register_vote(
    connection: &PgConnection,
    shortEventId: u8,
    userId: u8,
) -> Result<usize, diesel::result::Error> {
    diesel::insert(short_event_votes)
        .values((userId, shortEventId))
        .into_columns((short_event_votes::user_id, short_event_votes::event_id))
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
