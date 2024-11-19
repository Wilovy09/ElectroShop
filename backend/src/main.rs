use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use middlewares::jwt::validator;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
mod helpers;
mod middlewares;
mod models;
mod params;
mod responses;
mod routes;
use crate::responses::message::Messages;

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

    // Habilitar las restricciones de claves foráneas
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .expect("Error al habilitar las restricciones de claves foráneas");

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(validator);
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hc)
            .configure(routes::client::auth::config)
            .configure(routes::client::categories::config)
            .configure(routes::client::products::config)
            .configure(routes::client::sell::config)
            //Rutas Admin
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .configure(routes::admin::categories::config)
                    .configure(routes::admin::products::config)
                    .configure(routes::admin::sell::config),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

