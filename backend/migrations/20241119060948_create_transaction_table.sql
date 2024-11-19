-- Add migration script here
CREATE TABLE "Transaction" (
    id INTEGER PRIMARY KEY NOT NULL,
    id_sell INTEGER,
    product_name TEXT,
    id_product INTEGER,
    quantity INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_sell) REFERENCES Sell (id),
    FOREIGN KEY (id_product) REFERENCES Product (id)
);
