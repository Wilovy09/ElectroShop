use crate::{
    models::product::Product,
    params::{category::PartialCategoryParams, product::PartialProductParams},
    responses::message::Messages,
    AppState,
};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get).service(find).service(find_by_category);
}

#[get("/products")]
async fn get(state: Data<AppState>) -> HttpResponse {
    match sqlx::query_as::<_, Product>("SELECT * FROM Product")
        .fetch_all(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().json(Messages {
            message: "Products not found".to_string(),
        }),
    }
}

#[get("/products/{id}")]
async fn find(state: Data<AppState>, param: web::Path<PartialProductParams>) -> HttpResponse {
    match sqlx::query_as::<_, Product>("SELECT * FROM Product WHERE id = $1")
        .bind(param.id)
        .fetch_one(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().json(Messages {
            message: "Product not found".to_string(),
        }),
    }
}

#[get("/category/{id}/products")]
async fn find_by_category(
    state: Data<AppState>,
    param: web::Path<PartialCategoryParams>,
) -> HttpResponse {
    match sqlx::query_as::<_, Product>("SELECT * FROM Product WHERE category_id = $1")
        .bind(param.id)
        .fetch_all(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().json(Messages {
            message: "No products found in this category".to_string(),
        }),
    }
}
