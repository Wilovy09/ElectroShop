use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub image: String,
    pub description: String,
    pub price: f64,
    pub units: i64,
    pub deleted: Option<String>, // 0 false | 1 true
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CreateProduct {
    pub category_id: i64,
    pub name: String,
    pub image: String,
    pub description: String,
    pub price: f64,
    pub units: i64,
    pub deleted: Option<String>, // 0 false | 1 true
}
