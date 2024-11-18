use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}