use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use dotenv::dotenv;
mod responses;
use crate::responses::message::Messages;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct AppState {
    db: Pool<Sqlite>,
}

#[get("/hc")]
async fn hc() -> HttpResponse {
    HttpResponse::Ok().json(Messages {
        message: "ok".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("Unable to find DATABASE_URL");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al crear la conexión a la base de datos");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hc)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
