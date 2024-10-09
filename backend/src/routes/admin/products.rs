use crate::{
    models::product::CreateProduct,
    responses::message::ErrorMessages,
    AppState,
};
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use actix_web_grants::protect;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}

#[post("/products")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateProduct>) -> HttpResponse {
    let category_id = &body.category_id.clone();
    let name = &body.name.clone();
    let image = &body.image.clone();
    let price = &body.price.clone();
    let units = &body.units.clone();
    let deleted = &body.deleted.clone();
    match sqlx::query_as!(Product, "INSERT INTO Product (category_id, name, image, price, units, deleted) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", category_id, name, image, price, units, deleted)
        .fetch_one(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages{
            error_code: 401,
            message: "loquesea".to_string()
        })
    }
}

/*
#[delete("/products/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialCategorieParams>) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

#[put("/products/{id}")]
#[protect("Administrador")]
async fn edit(
    state: Data<AppState>,
    params: web::Path<PartialCategorieParams>,
    body: Json<CreateCategory>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok");
}
*/
