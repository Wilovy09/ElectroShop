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
    cfg.app_data(BearerConfig::default().realm("jwt"))
        .service(
            web::scope("/admin")
                .wrap(auth)
                .service(create)
                .service(delete)
                .service(edit),
        )
        .service(get);
}

#[post("/products")]
#[protect("Administrador")]
async fn create(state: Data<AppState>, body: Json<CreateCategory>) -> HttpResponse {
    HttpResponse::Ok().json("ok");
}

#[get("/products")]
#[protect("Administrador")]
async fn get(state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json("ok");
}

#[delete("/products/{id}")]
#[protect("Administrador")]
async fn delete(state: Data<AppState>, params: web::Path<PartialCategorieParams>) -> HttpResponse {
    HttpResponse::Ok().json("ok");
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
