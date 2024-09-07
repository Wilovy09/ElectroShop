use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
