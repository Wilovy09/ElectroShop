use crate::{models::transaction::TransactionSellResponse, responses::message::Messages, AppState};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use actix_web_grants::protect;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_sell_transactions);
}

#[get("/sell")]
#[protect("Administrador")]
async fn get_all_sell_transactions(state: Data<AppState>) -> HttpResponse {
    let result: Result<Vec<TransactionSellResponse>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT 
            s.total_amount,
            s.sell_date,
            t.product_name,
            t.id_sell,
            t.id_product,
            t.quantity
        FROM "Transaction" t
        JOIN "Sell" s ON t.id_sell = s.id 
        "#,
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            eprintln!("Error fetching data: {:?}", err);
            HttpResponse::InternalServerError().json(Messages {
                message: "Failed to fetch sell transactions".to_string(),
            })
        }
    }
}
