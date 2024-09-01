use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub image: String,
    pub price: f64,
    pub units: i32,
    pub deleted: bool,
}

