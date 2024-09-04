use crate::responses::message::Messages;
use actix_web::{get, web, HttpResponse};
use crate::models::role::Role;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[get("/hc")]
async fn health_check() -> HttpResponse {
    let message = Messages {
        message: "Server online!".to_string(),
    };
    HttpResponse::Ok().json(message)
}
