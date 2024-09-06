use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}
