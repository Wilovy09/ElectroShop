use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
mod helpers;
mod models;
mod params;
mod responses;
mod routes;
use dotenv::dotenv;
mod middlewares;
use actix_web_httpauth::{
    extractors::bearer::Config as BearerConfig, middleware::HttpAuthentication,
};
use middlewares::jwt::validator;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct AppState {
    db: Pool<Sqlite>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("Unable to find DATABASE_URL");    

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al crear la conexión a la base de dato");

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(validator);
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .configure(routes::client::auth::config)
            .configure(routes::client::products::config)
            .configure(routes::client::categories::config)
            // Admin
            .app_data(BearerConfig::default().realm("admin"))
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .configure(routes::admin::categories::config)
                    .configure(routes::admin::products::config)
                    .configure(routes::admin::roles::config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
