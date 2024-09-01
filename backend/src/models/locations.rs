use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Location {
    pub id: i64,
    pub user_id: i64,
    pub address: String,
    pub city: String,
    pub country: String,
}

