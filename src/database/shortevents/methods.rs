use diesel::dsl::sql;
use diesel::prelude::*;

use super::models::{InsertableShortEvent, ShortEvent, ShortEventVote, GroupedShortEvent};

use crate::database::schema::short_events::dsl::*;
use crate::database::schema::short_event_votes::dsl::*;

pub fn list(
    connection: &PgConnection,
) -> Result<Vec<(ShortEvent, Vec<ShortEventVote>)>, diesel::result::Error> {
    let short_event_list = short_events
        .filter(active.eq(true))
        .load::<ShortEvent>(connection)?;
    let vote_list = ShortEventVote::belonging_to(&short_event_list)
        .load::<ShortEventVote>(connection)?
        .grouped_by(&short_event_list);
    Ok(short_event_list.into_iter().zip(vote_list).collect())
}

pub fn get(
    connection: &PgConnection,
    target_short_event_id: i32,
) -> Result<Vec<(ShortEvent, Vec<ShortEventVote>)>, diesel::result::Error> {
    let target_short_event = short_events
        .find(target_short_event_id)
        .load::<ShortEvent>(connection)?;
    let vote_list = ShortEventVote::belonging_to(&target_short_event)
        .load::<ShortEventVote>(connection)?
        .grouped_by(&target_short_event);
    Ok(target_short_event.into_iter().zip(vote_list).collect())
}

pub fn insert(
    connection: &PgConnection,
    shortevent: &InsertableShortEvent,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(short_events)
        .values(shortevent)
        .execute(connection)
}

pub fn delete(
    connection: &PgConnection,
    shortevent_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::delete(short_events.filter(id.eq(shortevent_id))).execute(connection)
}

pub fn cleanup(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(short_events)
        .filter(active.eq(false))
        .execute(connection)
}

pub fn register_vote(
    connection: &PgConnection,
    event_vote: ShortEventVote,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(short_event_votes)
        .values(event_vote)
        .execute(connection)
}

pub fn check_votes(connection: &PgConnection) -> Result<usize, diesel::result::Error> {
    /*
     * NOTE: Proper support for GROUP BY () HAVING () is not yet available:
     *
     * https://github.com/diesel-rs/diesel/issues/210
     */
    match diesel::sql_query("SELECT event_id FROM short_event_votes GROUP BY event_id HAVING COUNT(*) >= 3;")
        .load::<GroupedShortEvent>(connection)
    {
        Ok(events) => {
            let event_ids: Vec<i32> = events.iter().map(|event| event.event_id).collect();
            diesel::delete(short_events)
                .filter(id.eq_any(event_ids))
                .execute(connection)
        }
        Err(e) => Err(e),
    }
}
