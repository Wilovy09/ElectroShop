-- Add migration script here
CREATE TABLE Category (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE
);
