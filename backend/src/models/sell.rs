use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Sell {
    pub id: i64,
    pub user_is: i64,
    pub total_amount: f64,
    pub sell_date: NaiveDateTime,
}
