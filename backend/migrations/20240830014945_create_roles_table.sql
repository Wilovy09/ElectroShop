-- Add migration script here

CREATE TABLE Role (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name_role TEXT NOT NULL UNIQUE CHECK(name_role IN ('Administrator', 'User'))
);
