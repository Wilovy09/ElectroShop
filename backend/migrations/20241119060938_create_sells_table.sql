-- Add migration script here
CREATE TABLE Sell (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    total_amount REAL NOT NULL,
    sell_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES User (id)
);
