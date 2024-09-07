use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRole {
    pub name_role: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name_role: String,
}
