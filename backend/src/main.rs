use actix_web::{web::Data, App, HttpServer};
mod helpers;
mod models;
mod params;
mod responses;
mod routes;
use dotenv::dotenv;
mod middlewares;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct AppState {
    db: Pool<Sqlite>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("No se encontro DATABASE_URL");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al crear la conexión a la base de dato");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .configure(routes::roles::config)
            .configure(routes::auth::config)
            .configure(routes::categories::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
