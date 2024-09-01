-- Add migration script here

CREATE TABLE Cart (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  userId INTEGER NOT NULL,
  totalAmount REAL NOT NULL DEFAULT 0,
  FOREIGN KEY (userId) REFERENCES User(id)
);
