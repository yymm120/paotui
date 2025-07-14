#![allow(unused)]
use crate::runtime::{block_on, default_runtime};
use futures::TryStreamExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Acquire, Connection, Error, Executor, PgConnection, Pool, Postgres, Row};
use std::sync::OnceLock;
use tokio::task;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
  pub(crate) user_id: Uuid,
  pub user_phone: String,
}

static INSTANCE: OnceLock<Pool<Postgres>> = OnceLock::new();
pub async fn pool() -> &'static Pool<Postgres> {
  INSTANCE.get_or_init(|| -> Pool<Postgres> {
    block_on(default_pool()).expect("pool initialization failed")
  })
}

pub async fn default_pool() -> anyhow::Result<Pool<Postgres>> {
  let pool = PgPoolOptions::new()
    .max_connections(1)
    .connect("postgres://postgres:postgres@localhost/paotui")
    .await?;
  Ok(pool)
}

/// 创建单个连接 (不可变)
pub async fn conn_immutable() -> anyhow::Result<PgConnection> {
  let conn = PgConnection::connect("postgres://postgres:postgres@localhost/paotui").await?;
  Ok(conn)
}

/// 创建单个连接 (可变)
pub async fn conn_mutable() -> anyhow::Result<PgConnection> {
  let mut conn = PgConnection::connect("postgres://postgres:postgres@localhost/paotui").await?;
  Ok(conn)
}

pub async fn mapping_instance_case1() -> anyhow::Result<User> {
  let mut conn = conn_mutable().await?;
  // postgres 使用 $1 作为占位符, 切记不要使用 ? 做占位符
  let mut user = sqlx::query_as::<_, User>("SELECT * FROM user_table WHERE user_phone = $1")
    .bind("8613812345678".to_string())
    .fetch_one(&mut conn)
    .await?;

  Ok(user)
}

pub async fn mapping_instance_case2() -> anyhow::Result<User> {
  let mut conn = conn_mutable().await?;
  let mut stream = sqlx::query("SELECT * FROM user_table")
    .map(move |row: PgRow| -> anyhow::Result<User> {
      let id = row.try_get::<Uuid, usize>(0)?;
      let phone = row.try_get(1)?;
      Ok(User {
        user_id: id,
        user_phone: phone,
      })
    })
    .fetch(&mut conn);
  let user = stream.try_next().await?.unwrap()?;
  Ok(user)
}

async fn query_all_user() -> anyhow::Result<()> {
  let pool = pool().await;
  let conn = pool.acquire().await?;
  let users = sqlx::query_as!(User, "SELECT * FROM user_table")
    .fetch_all(pool)
    .await?;
  Ok(())
}

async fn query_user_by_id(id: Uuid) -> anyhow::Result<User> {
  let pool = pool().await;
  let user = sqlx::query_as!(User, "SELECT * FROM user_table WHERE user_id = $1", id)
    .fetch_one(pool)
    .await?;
  Ok(user)
}

async fn query_user_by_phone(phone: String) -> anyhow::Result<User> {
  let pool = pool().await;
  let user = sqlx::query_as!(
    User,
    "SELECT * FROM user_table WHERE user_phone = $1",
    phone
  )
  .fetch_one(pool)
  .await?;
  Ok(user)
}

async fn update_user(user: User) -> anyhow::Result<bool> {
  let pool = pool().await;
  let query = sqlx::query!(
    r#"
UPDATE user_table
SET user_phone = $2
WHERE user_id = $1
    "#,
    user.user_id,
    user.user_phone
  );
  let res = query.execute(pool).await?;
  // res.map(|_| true)
  match res.rows_affected() {
    1 => Ok(true),
    _ => Ok(false),
  }
}

async fn create_user(user: User) -> anyhow::Result<bool> {
  let pool = pool().await;
  let query = sqlx::query!(
    "INSERT INTO user_table (user_id, user_phone) VALUES ($1, $2)",
    user.user_id,
    user.user_phone
  );
  let res = query.execute(pool).await?;
  match res.rows_affected() {
    1 => Ok(true),
    _ => Ok(false),
  }
}

async fn delete_user(user: User) -> anyhow::Result<bool> {
  let pool = pool().await;
  let query = sqlx::query!(
    "DELETE FROM user_table WHERE user_phone = $1;",
    user.user_phone
  );
  let res = query.execute(pool).await?;
  match res.rows_affected() {
    1 => Ok(true),
    _ => Ok(false),
  }
}
