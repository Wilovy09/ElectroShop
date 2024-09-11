use crate::middlewares::jwt::validator;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_grants::protect;
use actix_web_httpauth::{
    extractors::bearer::Config as BearerConfig, middleware::HttpAuthentication,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::with_fn(validator);
    cfg.app_data(BearerConfig::default().realm("jwt"))
        .service(web::scope("/admin").wrap(auth).service(create).service(get));
}

#[post("/categories")]
#[protect("Administrador")]
async fn create() -> HttpResponse {
    // TODO: Creación de cateogria
    HttpResponse::Ok().json("Hola")
}

#[get("/categories")]
#[protect("Administrador")]
async fn get() -> HttpResponse {
    // TODO: Ver todas las categorias
    HttpResponse::Ok().json("Hola")
}

#[delete("/categories/{id}")]
#[protect("Administrador")]
async fn delete() -> HttpResponse {
    // TODO: Eliminar una categoria
    HttpResponse::Ok().json("Hola")
}

#[put("/categories/{id}")]
#[protect("Administrador")]
async fn edit() -> HttpResponse {
    // TODO: Editar una categoria
    HttpResponse::Ok().json("Hola")
}
