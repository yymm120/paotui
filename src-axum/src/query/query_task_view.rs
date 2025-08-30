use crate::model::table::delivery_task::DeliveryTask;
use crate::model::view::task_view::TaskView;

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