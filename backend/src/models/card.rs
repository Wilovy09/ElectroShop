use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Card {
    pub id: i64,
    pub user_id: i64,
    pub card_number: String,
    pub expiration: String,
    pub security_code: i32,
    pub card_owner: String,
}

