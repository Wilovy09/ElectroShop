use actix_web::{App, HttpServer};
mod routes;
mod responses;
mod models;
// use models::{Role, User, Category, Product, Cart, CartProduct, Card, Location};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
