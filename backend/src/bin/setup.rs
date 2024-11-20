use dotenv::dotenv;
use sqlx::{sqlite::SqlitePoolOptions};

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Unable to find DATABASE_URL");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al crear la conexión a la base de datos");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("Error al habilitar claves foráneas");

    sqlx::query(r#"
    DROP TABLE IF EXISTS User;
    DROP TABLE IF EXISTS Category;
    DROP TABLE IF EXISTS Product;
    DROP TABLE IF EXISTS Sell;
    DROP TABLE IF EXISTS 'Transaction';
    CREATE TABLE User (
        id INTEGER PRIMARY KEY NOT NULL,
        role TEXT NOT NULL CHECK(role IN ('Administrador', 'Cliente')),
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME
    );
    CREATE TABLE Category (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT NOT NULL UNIQUE
    );
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
    CREATE TABLE Sell (
        id INTEGER PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL,
        total_amount REAL NOT NULL,
        sell_date DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES User (id)
    );
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
    "#)
        .execute(&pool)
        .await
        .expect("Error al limpiar base de datos");
}

