-- Add migration script here

CREATE TABLE Cards (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  userId INTEGER NOT NULL,
  cardNumber TEXT NOT NULL,
  expiration TEXT NOT NULL CHECK(LENGTH(expiration) = 5), -- Formato 'MM/YY'
  securityCode INTEGER NOT NULL CHECK(LENGTH(securityCode) = 3),
  cardOwner TEXT NOT NULL,
  FOREIGN KEY (userId) REFERENCES User(id)
);
