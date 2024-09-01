use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CartProduct {
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

