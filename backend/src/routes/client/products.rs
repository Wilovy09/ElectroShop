use crate::{models::product::Product, responses::message::ErrorMessages, AppState};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/products")]
async fn get(state: Data<AppState>) -> HttpResponse {
    match sqlx::query_as::<_, Product>("SELECT * FROM Product")
        .fetch_all(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().json(ErrorMessages {
            error_code: 404,
            message: "Products not found".to_string(),
        }),
    }
}

// #[get("/products/:id")]
// async fn getProduct(state: Data<AppState>, param: web::Path<i64>)
