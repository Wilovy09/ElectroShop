-- Add migration script here
CREATE TABLE User (
    id INTEGER PRIMARY KEY NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('Administrador', 'Cliente')),
    full_name TEXT,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME
);
