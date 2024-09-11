use actix_web::{
    post,
    web::{self},
    HttpResponse,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_categories);
}

#[post("/categories")]
async fn create_categories() -> HttpResponse {
    HttpResponse::Ok().json("Hola")
}
