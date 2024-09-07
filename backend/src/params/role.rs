use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct DeleteRoleParams {
    pub id: i64,
}
