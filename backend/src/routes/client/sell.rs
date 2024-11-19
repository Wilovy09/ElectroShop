use crate::{params::sell::PartialSell, responses::message::Messages, AppState};
use actix_web::{
    post,
    web::{self, Data, Json},
    HttpResponse,
};
use sqlx::query;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_sell);
}

#[post("/sell")]
async fn create_sell(state: Data<AppState>, body: Json<PartialSell>) -> HttpResponse {
    // Insertar en la tabla `Sell`
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

    // Insertar en la tabla `Transaction` por cada producto
    for product in &body.transactions {
        if let Err(err) = query(
            r#"
            INSERT INTO "Transaction" (id_sell, product_name, id_product, quantity)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(sell_id) // ID de la venta
        .bind(&product.product_name) // Nombre del producto
        .bind(product.id_product) // ID del producto
        .bind(product.quantity) // Cantidad
        .execute(&state.db)
        .await
        {
            eprintln!("Error inserting into Transaction: {:?}", err);
            return HttpResponse::InternalServerError().json(Messages {
                message: "Failed to create transaction".to_string(),
            });
        }
    }

    // Retornar No Content
    HttpResponse::NoContent().finish()
}
