// CREATE & DELETE
use crate::{
    models::category::{Category, CreateCategory},
    params::category::PartialCategoryParams,
    responses::message::Messages,
    AppState,
};
use actix_web::{
    delete, post,
    web::{self, Data, Json},
    HttpResponse,
};
use actix_web_grants::protect;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create).service(delete);
}

#[post("/category")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateCategory>) -> HttpResponse {
    let category_name = &body.name.clone();
    match sqlx::query_as!(
        Category,
        "INSERT INTO Category (name) VALUES ($1) RETURNING id, name",
        category_name
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(created_category) => HttpResponse::Ok().json(created_category),
        Err(_) => HttpResponse::Unauthorized().json(Messages {
            message: "Invalid role.".to_string(),
        }),
    }
}

#[delete("/categories/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialCategoryParams>) -> HttpResponse {
    match sqlx::query!("DELETE FROM Category WHERE id = $1", params.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(Messages {
            message: "Succesfully deleted".to_string(),
        }),
        Err(_) => HttpResponse::NotFound().json(Messages {
            message: "Category not found".to_string(),
        }),
    }
}
