use std::any::Any;
use crate::model::table::user::{CreateUser, UpdateUser, User};
use crate::query::error::Result;
use crate::utils::db::Db;
use std::str::FromStr;
use sqlx::{Database, Execute, Executor, Pool, Postgres, Transaction};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::Map;
use tracing::instrument;
use crate::query::delivery_app_user::IntoExecutor;
use crate::utils::time::now_time;

pub enum Connection<'a> {
  Pool(Db),
  Transaction(&'a mut Transaction<'a, Postgres>),
}


fn query_by_user_phone(user_phone: String) -> Map<Postgres, fn(PgRow) -> Result<User>, PgArguments> {
  sqlx::query_as!(User,"SELECT * FROM public.user WHERE telephone = $1",user_phone)
}

fn query_by_user_id(user_id: i64) -> Map<Postgres, fn(PgRow) -> Result<User>, PgArguments> {
  sqlx::query_as!(User,"SELECT * FROM public.user WHERE user_id = $1",user_id)
}




// #[instrument]
pub async fn query_user_by_phone<'a, E>(
  executor: E,
  user_phone: String
) -> Result<User>
where
  E: Executor<'a, Database = Postgres>,
{
  let user = query_by_user_phone(user_phone.clone())
    .fetch_one(executor).await?;
  Ok(user)
}

// #[instrument]
pub async fn query_user_by_id(db: &Db, user_id: i64) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    "SELECT * FROM public.user WHERE user_id = $1",
    user_id
  )
  .fetch_one(db)
  .await?;
  Ok(user)
}

// #[instrument]
pub async fn query_user(db: &Db, user_id: i64, user_phone: String) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    "SELECT * FROM public.user WHERE user_id = $1 AND telephone = $2",
    user_id,
    user_phone
  )
  .fetch_one(db)
  .await?;
  Ok(user)
}


// #[instrument]
pub async fn create_user(transaction: &mut Transaction<'_, Postgres>, create_user: CreateUser) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    r#"
        INSERT INTO public.user (username, password, telephone)
        VALUES ($1, $2, $3)
        ON CONFLICT (telephone) DO NOTHING
        RETURNING *
        "#,
    create_user.username,
    create_user.password,
    create_user.telephone,
  )
    .fetch_one(&mut **transaction)
    .await?;
  Ok(user)
}

pub async fn delete_user(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, user_id: i64) -> Result<User> {
  let user = sqlx::query_as!(User, r#"
    DELETE FROM public.user WHERE user_id = $1 RETURNING *;
  "#,
  user_id
  ).fetch_one(&mut **transaction).await?;
  Ok(user)
}

pub async fn update_user(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, update_user: UpdateUser) -> Result<User> {

  let user = sqlx::query_as!(User, r#"
    UPDATE public.user
        SET
        username = COALESCE($2, username),
        password = COALESCE($3, password),
        telephone = COALESCE($4, telephone),
        updated_at = COALESCE($5, updated_at)
    WHERE user_id = $1
    RETURNING *;
  "#,
    update_user.user_id,
    update_user.username,
    update_user.password,
    update_user.telephone,
    now_time()
  ).fetch_one(&mut **transaction).await?;

  Ok(user)
}