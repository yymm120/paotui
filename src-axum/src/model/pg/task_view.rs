use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{Date, OffsetDateTime, PrimitiveDateTime};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TaskView {
  pub id: Option<String>,
  pub tag: Option<String>,
  pub items: Option<String>,
  pub note: Option<String>,
  pub status: Option<String>,
  pub priority: Option<String>,
  pub estimated_income: Option<String>,
  pub username_send: Option<String>,
  pub username_receive: Option<String>,
  pub username_delivery: Option<String>,
  pub user_id_created_by: Option<String>,
  pub telephone_send: Option<String>,
  pub telephone_receive: Option<String>,
  pub telephone_delivery: Option<String>,
  pub user_id_delivery_by: Option<String>,
  pub create_time: Option<PrimitiveDateTime>,
  pub arrived_time: Option<PrimitiveDateTime>,

  // 起始地址信息
  pub start_poiname: Option<String>,
  pub start_cityname: Option<String>,
  pub start_poiaddress: Option<String>,
  pub start_lat: Option<f64>,
  pub start_lng: Option<f64>,

  // 目的地址信息
  pub end_poiname: Option<String>,
  pub end_cityname: Option<String>,
  pub end_poiaddress: Option<String>,
  pub end_lat: Option<f64>,
  pub end_lng: Option<f64>,
  // // 当前地址信息
  // pub current_poiname: Option<String>,
  // pub current_cityname: Option<String>,
  // pub current_poiaddress: Option<String>,
  // pub current_lat: Option<f64>,
  // pub current_lng: Option<f64>,
}

pub async fn query_task_view_by_status(
  db: &sqlx::PgPool,
  status: String,
) -> crate::query::error::Result<Vec<TaskView>> {
  let delivery_task = sqlx::query_as!(
    TaskView,
    "SELECT * FROM task_view WHERE status = $1",
    status
  )
    .fetch_all(db)
    .await?;

  Ok(delivery_task)
}

pub async fn query_task_view_by_id(
  db: &sqlx::PgPool,
  id: String,
) -> crate::query::error::Result<TaskView> {
  let delivery_task = sqlx::query_as!(TaskView, "SELECT * FROM task_view WHERE id = $1", id)
    .fetch_one(db)
    .await?;

  Ok(delivery_task)
}

pub async fn query_task_view_by_owner(
  db: &sqlx::PgPool,
  id: i32,
) -> crate::query::error::Result<Vec<TaskView>> {
  let delivery_task = sqlx::query_as!(
    TaskView,
    "SELECT * FROM task_view WHERE user_id_delivery_by = $1",
    id
  )
    .fetch_all(db)
    .await?;

  Ok(delivery_task)
}