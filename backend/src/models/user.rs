use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub role: String,
    pub full_name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PartialUser {
    pub id: i64,
    pub role: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DataUser {
    pub id: i64,
    pub role: String,
    pub email: String,
    pub full_name: Option<String>,
}
