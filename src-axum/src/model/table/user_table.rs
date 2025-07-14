use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct UserTable {
  pub user_id: Uuid,
  pub user_phone: String,
}
