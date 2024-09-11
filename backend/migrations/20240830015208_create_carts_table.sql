-- Add migration script here

CREATE TABLE Cart (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_id INTEGER NOT NULL,
  total_amount REAL NOT NULL DEFAULT 0,
  FOREIGN KEY (user_id) REFERENCES User(id)
);
