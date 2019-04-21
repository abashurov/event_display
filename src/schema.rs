table! {
    display_tokens (id) {
        id -> Int4,
        token -> Varchar,
    }
}

table! {
    event_assignees (event_id, user_id) {
        event_id -> Int4,
        user_id -> Int4,
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
        type_ -> Int2,
        group_id -> Int4,
    }
}

table! {
    short_events (id) {
        id -> Int4,
        user_id -> Int4,
        description -> Varchar,
        time_begin -> Time,
        active -> Bool,
    }
}

table! {
    short_event_votes (id) {
        id -> Int4,
        user_id -> Int4,
        event_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        adlogin -> Varchar,
        display_name -> Varchar,
        absent -> Bool,
        password -> Varchar,
        superuser -> Bool,
        availability -> Int2,
    }
}

joinable!(event_assignees -> events (event_id));
joinable!(event_assignees -> users (user_id));
joinable!(events -> event_groups (group_id));
joinable!(short_event_votes -> short_events (event_id));
joinable!(short_event_votes -> users (user_id));
joinable!(short_events -> users (user_id));

allow_tables_to_appear_in_same_query!(
    display_tokens,
    event_assignees,
    event_groups,
    events,
    short_events,
    short_event_votes,
    users,
);
