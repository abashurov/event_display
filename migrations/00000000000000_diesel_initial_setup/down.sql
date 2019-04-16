-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();

DROP TABLE display_tokens;
DROP TABLE short_event_votes;
DROP TABLE short_events;
DROP TABLE event_assignees;
DROP TABLE users;
DROP TABLE events;
DROP TABLE event_groups;
