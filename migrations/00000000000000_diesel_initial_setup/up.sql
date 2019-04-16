-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE event_groups (
    id SERIAL NOT NULL PRIMARY KEY,
    display_name VARCHAR(128) NOT NULL
);

CREATE TABLE events (
    id SERIAL NOT NULL PRIMARY KEY,
    time_from TIME NOT NULL,
    time_to TIME NOT NULL,
    day SMALLINT NOT NULL,
    type SMALLINT NOT NULL, --0x0 -> chats, 0x1 -> calls, others -> not implemented
    group_id INTEGER NOT NULL REFERENCES event_groups(id) ON DELETE RESTRICT,
    CONSTRAINT valid_type CHECK (type < 2)
);

CREATE TABLE users (
    id SERIAL NOT NULL PRIMARY KEY,
    adlogin VARCHAR(128) NOT NULL UNIQUE,
    display_name VARCHAR(256) NOT NULL DEFAULT 'Default engineer',
    absent BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(128) NOT NULL,
    superuser BOOLEAN NOT NULL DEFAULT FALSE,
    availability SMALLINT NOT NULL DEFAULT 0, --Bitmask 0x0(sun)(sat)(fri)(thu)(wed)(tue)(mon)
    CONSTRAINT valid_availability CHECK (availability < 256)
);

CREATE TABLE event_assignees (
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE, --Reasonable to assume nobody will
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, --be happy to remove it manually
    PRIMARY KEY (event_id, user_id) --Every user cannot be assigned to each event more than once
);

CREATE TABLE short_events (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    description VARCHAR(128) NOT NULL,
    time_begin TIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (user_id, active)
);

CREATE TABLE short_event_votes (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    event_id INTEGER NOT NULL REFERENCES short_events(id) ON DELETE CASCADE,
    UNIQUE (id, user_id, event_id)
);

CREATE TABLE display_tokens (
    id SERIAL NOT NULL PRIMARY KEY,
    token VARCHAR(128) NOT NULL
);
