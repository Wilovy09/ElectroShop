use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct PartialCategoryParams {
    pub id: i64,
}
