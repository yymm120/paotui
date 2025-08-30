use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::PrimitiveDateTime;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
  pub user_id: i64,
  pub user_phone: String,
  pub delivery_role_id: Option<i64>,
  pub create_at: PrimitiveDateTime
}