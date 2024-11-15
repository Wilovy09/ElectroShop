-- Add migration script here
CREATE TABLE Sell (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER,
    total_amount REAL DEFAULT 0.0,
    status TEXT DEFAULT "pending" CHECK(status IN ("pending", "done")),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES User(id)
);
