use crate::{
    models::{product::CreateProduct, Product}, params::PartialProductParams, responses::message::ErrorMessages, AppState
};
use actix_web::{
    delete, post, put,
    web::{self, Data, Json},
    HttpResponse,
};
use actix_web_grants::protect;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(delete).service(edit);
}

#[post("/products")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateProduct>) -> HttpResponse {
    let category_id = &body.category_id.clone();
    let name = &body.name;
    let image = &body.image;
    let price = &body.price;
    let units = &body.units;
    let deleted = &body.deleted;
    match sqlx::query_as!(
        Product, 
        "INSERT INTO Product (category_id, name, image, price, units, deleted) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", 
        category_id, 
        name, 
        image, 
        price, 
        units, 
        deleted
    )
        .fetch_one(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages{
            error_code: 401,
            message: "Invalid token.".to_string()
        })
    }
}

#[delete("/products/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialProductParams>) -> HttpResponse {
    let id = &params.id;
    match sqlx::query_as!(
        Product,
        "DELETE FROM Product WHERE id = $1 RETURNING *",
        id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
            error_code: 401,
            message: "Invalid token.".to_string(),
        }),
    }
}

#[put("/products/{id}")]
#[protect("Administrador")]
async fn edit(
    state: Data<AppState>,
    params: web::Path<PartialProductParams>,
    body: Json<CreateProduct>,
) -> HttpResponse {
    let category_id = &body.category_id;
    let name = &body.name;
    let image = &body.image;
    let price = &body.price;
    let units = &body.units;
    let deleted = &body.deleted;
    let id = &params.id;

    match sqlx::query_as!(
        Product, 
        "UPDATE Product SET category_id = $1, name = $2, image = $3, price = $4, units = $5, deleted = $6 WHERE id = $7 RETURNING *", 
        category_id, 
        name, 
        image, 
        price, 
        units, 
        deleted, 
        id
    )
        .fetch_one(&state.db)
        .await
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
            error_code: 401,
            message: "Invalid token.".to_string(),
        })
    }
}
