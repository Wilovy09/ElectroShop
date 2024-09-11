-- Add migration script here

CREATE TABLE Location (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_id INTEGER NOT NULL,
  address TEXT NOT NULL,
  city TEXT NOT NULL,
  country TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES User(id)
);
