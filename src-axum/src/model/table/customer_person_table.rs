use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPersonTable {
  pub id: i64,
  pub password_hash: Option<String>,
  pub username: Option<String>,
  pub telephone: String,
  pub money: Option<Decimal>,
  pub status: Option<String>,
  pub create_at: PrimitiveDateTime,
  pub update_at: Option<PrimitiveDateTime>,
  pub delete_at: Option<PrimitiveDateTime>,
}