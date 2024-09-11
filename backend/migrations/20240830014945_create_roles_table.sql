-- Add migration script here

CREATE TABLE Role (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name_role TEXT NOT NULL UNIQUE
);
