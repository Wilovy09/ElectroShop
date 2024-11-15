// List
use crate::{
	models::category::Category, responses::message::Messages,
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

#[get("/categories")]
async fn get(state: Data<AppState>) -> HttpResponse {
	match sqlx::query_as::<_, Category>("SELECT * FROM Category")
		 .fetch_all(&state.db)
		 .await
	{
		 Ok(category) => HttpResponse::Ok().json(category),
		 Err(_) => HttpResponse::NotFound().json(Messages {
			  message: "Not categories".to_string(),
		 }),
	}
}
