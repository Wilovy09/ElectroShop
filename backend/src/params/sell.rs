use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionInput {
    pub product_name: String,
    pub id_product: i64,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct PartialSell {
    pub user_id: i64,
    pub total_amount: f64,
    pub transactions: Vec<TransactionInput>,
}
