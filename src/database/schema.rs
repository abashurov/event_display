table! {
    event_assignees (event_id, user_name) {
        event_id -> Int4,
        user_name -> Varchar,
    }
}

table! {
    event_groups (id) {
        id -> Int4,
        display_name -> Varchar,
    }
}

table! {
    events (id) {
        id -> Int4,
        time_from -> Time,
        time_to -> Time,
        day -> Int2,
        #[sql_name = "type"]
        event_type -> Int2,
        group_id -> Int4,
        display_name -> Varchar,
    }
}

table! {
    short_events (id) {
        id -> Int4,
        user_name -> Varchar,
        description -> Varchar,
        time_begin -> Time,
        active -> Bool,
    }
}

table! {
    short_event_votes (id) {
        id -> Int4,
        user_name -> Varchar,
        event_id -> Int4,
    }
}

table! {
    users (adlogin) {
        adlogin -> Varchar,
        display_name -> Varchar,
        absent -> Bool,
        password -> Varchar,
        role -> Int2,
        availability -> Int2,
    }
}

joinable!(event_assignees -> events (event_id));
joinable!(event_assignees -> users (user_name));
joinable!(events -> event_groups (group_id));
joinable!(short_event_votes -> short_events (event_id));
joinable!(short_event_votes -> users (user_name));
joinable!(short_events -> users (user_name));

allow_tables_to_appear_in_same_query!(
    event_assignees,
    event_groups,
    events,
    short_events,
    short_event_votes,
    users,
);
