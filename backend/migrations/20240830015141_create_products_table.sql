-- Add migration script here

CREATE TABLE Product (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  categoryId INTEGER NOT NULL,
  name TEXT NOT NULL,
  image TEXT NOT NULL, -- Almacena la imagen en formato Base64
  price REAL NOT NULL,
  units INTEGER NOT NULL CHECK(units >= 0),
  deleted INTEGER NOT NULL DEFAULT 0 CHECK(deleted IN (0, 1)),
  FOREIGN KEY (categoryId) REFERENCES Category(id)
);
