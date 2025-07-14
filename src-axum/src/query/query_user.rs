use crate::model::table::user_table::UserTable;
use crate::utils::db::Db;
use std::str::FromStr;
use tracing::instrument;
use uuid::Uuid;

#[instrument]
pub async fn query_user_by_phone(db: &Db, user_phone: String) -> Option<UserTable> {
  // let user_phone = login_body.user_phone;
  let exist_user = sqlx::query_as!(
    UserTable,
    "SELECT * FROM user_table WHERE user_phone = $1",
    user_phone
  )
  .fetch_optional(db)
  .await
  .expect("FAILED: Sql 查询失败, 可能数据库连接异常.");

  exist_user
}


pub async fn create_user(db: &Db, user_id: String, user_phone: String) -> sqlx::Result<UserTable> {
  // 这里需要开启事务
  let mut transaction = db.begin().await?;
  let res = sqlx::query_as!(
    UserTable,
    "INSERT INTO user_table (user_id, user_phone) VALUES ($1, $2) RETURNING *",
    Uuid::from_str(&user_id).unwrap_or(Uuid::default()),
    user_phone
  )
  .fetch_one(db)
  .await;
  transaction.commit().await?;
  res
}
