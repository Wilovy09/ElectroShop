CREATE TABLE Product (
    id INTEGER PRIMARY KEY NOT NULL,
    category_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    image TEXT NOT NULL, -- Base64
    description TEXT NOT NULL,
    price REAL NOT NULL,
    units INTEGER NOT NULL,
    deleted TEXT DEFAULT 'false' CHECK(deleted IN ('false', 'true')),
    FOREIGN KEY (category_id) REFERENCES Category (id) ON DELETE CASCADE
);
