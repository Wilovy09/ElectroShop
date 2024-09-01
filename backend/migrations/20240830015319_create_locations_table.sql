-- Add migration script here

CREATE TABLE Location (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  userId INTEGER NOT NULL,
  address TEXT NOT NULL,
  city TEXT NOT NULL,
  country TEXT NOT NULL,
  FOREIGN KEY (userId) REFERENCES User(id)
);
