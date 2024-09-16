use crate::{
    middlewares::jwt::validator,
    models::category::{Category, CreateCategory},
    params::PartialCategorieParams,
    responses::message::{ErrorMessages, Messages},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};
use actix_web_grants::protect;
use actix_web_httpauth::{
    extractors::bearer::Config as BearerConfig, middleware::HttpAuthentication,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::with_fn(validator);
    cfg.app_data(BearerConfig::default().realm("jwt")).service(
        web::scope("/admin")
            .wrap(auth)
            .service(create)
            .service(get)
            .service(delete)
            .service(edit),
    );
}

#[post("/categories")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateCategory>) -> HttpResponse {
    let category_name = &body.name.clone();
    match sqlx::query!(
        "INSERT INTO Category (name) VALUES ($1) RETURNING id, name",
        category_name
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(Messages {
            message: "Category created".to_string(),
        }),
        Err(_) => HttpResponse::Unauthorized().json(ErrorMessages {
            error_code: 401,
            message: "Invalid role.".to_string(),
        }),
    }
}

#[get("/categories")]
#[protect("Administrador")]
async fn get(state: Data<AppState>) -> HttpResponse {
    match sqlx::query_as::<_, Category>("SELECT * FROM Category")
        .fetch_all(&state.db)
        .await
    {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::NotFound().json(ErrorMessages {
            error_code: 404,
            message: "Not categories".to_string(),
        }),
    }
}

#[delete("/categories/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialCategorieParams>) -> HttpResponse {
    println!("{}", params.id);
    match sqlx::query!("DELETE FROM Category WHERE id = $1", params.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(_) => HttpResponse::NotFound().json(ErrorMessages {
            error_code: 404,
            message: "Category not found".to_string(),
        }),
    }
}

#[put("/categories/{id}")]
#[protect("Administrador")]
async fn edit(
    state: Data<AppState>,
    params: web::Path<PartialCategorieParams>,
    body: Json<CreateCategory>,
) -> HttpResponse {
    match sqlx::query_as::<_, Category>(
        "UPDATE Category SET name = $1 WHERE id = $2 RETURNING id, name",
    )
    .bind(&body.name)
    .bind(params.id)
    .fetch_one(&state.db)
    .await
    {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(_) => HttpResponse::NotFound().json(ErrorMessages {
            error_code: 404,
            message: "Category not found.".to_string(),
        }),
    }
}
