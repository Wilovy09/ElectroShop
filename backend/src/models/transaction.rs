use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TransactionSellResponse {
    pub total_amount: f64,
    pub sell_date: String,
    pub product_name: String,
    pub id_sell: i64,
    pub id_product: i64,
    pub quantity: i32,
}
