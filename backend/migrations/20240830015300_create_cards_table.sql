-- Add migration script here

CREATE TABLE Cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  card_number TEXT NOT NULL,
  expiration TEXT NOT NULL CHECK(LENGTH(expiration) = 5), -- Formato 'MM/YY'
  security_code INTEGER NOT NULL CHECK(LENGTH(security_code) = 3),
  card_owner TEXT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES User(id)
);
