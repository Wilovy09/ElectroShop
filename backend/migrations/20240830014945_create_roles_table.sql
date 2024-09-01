-- Add migration script here

CREATE TABLE Role (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nameRole TEXT NOT NULL CHECK(nameRole IN ('Administrator', 'User'))
);
