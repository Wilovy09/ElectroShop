use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
