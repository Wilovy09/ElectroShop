use crate::{
    models::category::Category, params::PartialCategorieParams, responses::message::ErrorMessages,
    AppState,
};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/category/{id}")]
async fn get(state: Data<AppState>, params: web::Path<PartialCategorieParams>) -> HttpResponse {
    match sqlx::query_as::<_, Category>("SELECT * FROM Category WHERE id = $1")
        .bind(params.id)
        .fetch_one(&state.db)
        .await
    {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(_) => HttpResponse::NotFound().json(ErrorMessages {
            error_code: 404,
            message: "Not categories".to_string(),
        }),
    }
}
