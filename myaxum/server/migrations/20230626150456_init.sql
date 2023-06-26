-- Add migration script here
create table testdb2 (
    id  SERIAL PRIMARY KEY,
    text    TEXT NOT NULL,
    completed   BOOLEAN NOT NULL DEFAULT FALSE
);