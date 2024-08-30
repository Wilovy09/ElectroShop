use actix_web::{get, http::header, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[get("/hc")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .body(
            r#"
        {
            "healthCheck": "Ok"
        }
        "#,
        )
}
