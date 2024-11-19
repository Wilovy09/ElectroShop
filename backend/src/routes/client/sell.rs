use crate::{
    models::transaction::TransactionSellResponse, params::sell::PartialSell,
    responses::message::Messages, AppState,
};
use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
use sqlx::query;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_sell).service(get_sell_transactions);
}

#[post("/sell")]
async fn create_sell(state: Data<AppState>, body: Json<PartialSell>) -> HttpResponse {
    let sell_id: i64 = match sqlx::query_scalar(
        r#"
        INSERT INTO Sell (user_id, total_amount)
        VALUES ($1, $2)
        RETURNING id
        "#,
    )
    .bind(body.user_id)
    .bind(body.total_amount)
    .fetch_one(&state.db)
    .await
    {
        Ok(id) => id,
        Err(err) => {
            eprintln!("Error inserting into Sell: {:?}", err);
            return HttpResponse::InternalServerError().json(Messages {
                message: "Failed to create sell".to_string(),
            });
        }
    };

    for product in &body.transactions {
        if let Err(err) = query(
            r#"
            INSERT INTO "Transaction" (id_sell, product_name, id_product, quantity)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(sell_id)
        .bind(&product.product_name)
        .bind(product.id_product)
        .bind(product.quantity)
        .execute(&state.db)
        .await
        {
            eprintln!("Error inserting into Transaction: {:?}", err);
            return HttpResponse::InternalServerError().json(Messages {
                message: "Failed to create transaction".to_string(),
            });
        }
    }

    HttpResponse::NoContent().finish()
}

#[get("/sell/{user_id}")]
async fn get_sell_transactions(state: Data<AppState>, user_id: web::Path<i64>) -> HttpResponse {
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
        WHERE s.user_id = $1
        "#,
    )
    .bind(user_id.into_inner())
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
