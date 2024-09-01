use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Cart {
    pub id: i64,
    pub user_id: i64,
    pub total_amount: f64,
}

