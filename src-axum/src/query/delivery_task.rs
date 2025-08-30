use crate::model::table::delivery_task::DeliveryTask;
use crate::model::task::{DeliveryTaskCreateData, DeliveryTaskUpdateData, Task};
use crate::query::error::{QueryError, Result};
use crate::utils::time::beijing_time;
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};
use tracing::debug;

pub async fn query_delivery_task_by_id(db: &sqlx::PgPool, id: String) -> Result<DeliveryTask> {
  let delivery_task = sqlx::query_as!(
    DeliveryTask,
    "SELECT * FROM delivery_task WHERE id = $1",
    id
  )
  .fetch_one(db)
  .await?;

  Ok(delivery_task)
}

pub async fn query_delivery_task_list_by_status(
  db: &sqlx::PgPool,
  status: String,
) -> Result<Vec<DeliveryTask>> {
  let delivery_task = sqlx::query_as!(
    DeliveryTask,
    "SELECT * FROM delivery_task WHERE status = $1",
    status
  )
  .fetch_all(db)
  .await?;

  Ok(delivery_task)
}

pub async fn delete_delivery_task_by_id(
  db: &sqlx::PgPool,
  id: String,
) -> Result<DeliveryTask> {
  let delivery_task = sqlx::query_as!(
    DeliveryTask,
    r#"
        DELETE FROM delivery_task
        WHERE id = $1
        RETURNING *
        "#,
    id
  )
  .fetch_one(db)
  .await?;
  Ok(delivery_task)
}

pub async fn update_delivery_task_set_null(
  db: &sqlx::PgPool,
  update_data: DeliveryTask,
) -> Result<DeliveryTask> {
  let delivery_task = sqlx::query_as!(
    DeliveryTask,
    r#"
        UPDATE delivery_task
        SET
            /* 2 */ create_time = $2,
            /* 3 */ arrived_time = $3 ,
            /* 4 */ address_start_id = $4,
            /* 5 */ address_end_id = $5,
            /* 6 */ tag = $6,
            /* 7 */ items = $7,
            /* 8 */ note = $8,
            /* 9 */ status = $9,
            /* 10 */ priority = $10,
            /* 11 */ estimated_income = $11,
            /* 12 */ username_send = $12,
            /* 13 */ username_receive = $13,
            /* 14 */ username_delivery = $14,
            /* 15 */ telephone_send = $15,
            /* 16 */ telephone_receive = $16,
            /* 17 */ telephone_delivery = $17,
            /* 18 */ user_id_delivery_by = $18,
            /* 19 */ user_id_created_by = $19
        WHERE id = $1
        RETURNING *
        "#,
    /* 1 */ update_data.id,
    /* 2 */ update_data.create_time, // arrived_time 送达时间
    /* 3 */ update_data.arrived_time, // arrived_time 送达时间
    /* 4 */ update_data.address_start_id, // 起始地址
    /* 5 */ update_data.address_end_id, // 结束地址
    /* 6 */ update_data.tag, // tag
    /* 7 */ update_data.items, // 物品
    /* 8 */ update_data.note, // 备注
    /* 9 */ update_data.status, // 状态
    /* 10 */ update_data.priority, // 优先级
    /* 11 */ update_data.estimated_income, // 预计收入
    /* 12 */ update_data.username_send, // 发送者
    /* 13 */ update_data.username_receive, // 接收者
    /* 14 */ update_data.username_delivery, // 配送者
    /* 15 */ update_data.telephone_send, // 发送者电话
    /* 16 */ update_data.telephone_receive, // 接收者电话
    /* 17 */ update_data.telephone_delivery, // 配送者电话
    /* 18 */ update_data.user_id_delivery_by, // 配送者id
    /* 19 */ update_data.user_id_created_by, // 谁创建的
  )
  .fetch_one(db)
  .await?;
  Ok(delivery_task)
}

pub async fn update_delivery_task_non_null(
  db: &sqlx::PgPool,
  update_data: DeliveryTaskUpdateData,
) -> Result<DeliveryTask> {
  // 开启事务
  let mut transaction = db.begin().await?;
  let delivery_task = sqlx::query_as!(
    DeliveryTask,
    r#"
        UPDATE delivery_task
        SET
            /* 2 */ create_time = COALESCE($2, create_time),
            /* 3 */ arrived_time = COALESCE($3,arrived_time ),
            /* 4 */ address_start_id = COALESCE($4, address_start_id),
            /* 5 */ address_end_id = COALESCE($5, address_end_id),
            /* 6 */ tag = COALESCE($6, tag),
            /* 7 */ items = COALESCE($7, items),
            /* 8 */ note = COALESCE($8, note),
            /* 9 */ status = COALESCE($9, status),
            /* 10 */ priority = COALESCE($10, priority),
            /* 11 */ estimated_income = COALESCE($11, estimated_income),
            /* 12 */ username_send = COALESCE($12, username_send),
            /* 13 */ username_receive = COALESCE($13, username_receive),
            /* 14 */ username_delivery = COALESCE($14, username_delivery),
            /* 15 */ telephone_send = COALESCE($15, telephone_send),
            /* 16 */ telephone_receive = COALESCE($16, telephone_receive),
            /* 17 */ telephone_delivery = COALESCE($17, telephone_delivery),
            /* 18 */ user_id_delivery_by = COALESCE($18, user_id_delivery_by),
            /* 19 */ user_id_created_by = COALESCE($19, user_id_created_by)
        WHERE id = $1
        RETURNING *
        "#,
    /* 1 */ update_data.id,
    /* 2 */ update_data.create_time, // arrived_time 送达时间
    /* 3 */ update_data.arrived_time, // arrived_time 送达时间
    /* 4 */ update_data.address_start_id, // 起始地址
    /* 5 */ update_data.address_end_id, // 结束地址
    /* 6 */ update_data.tag, // tag
    /* 7 */ update_data.items, // 物品
    /* 8 */ update_data.note, // 备注
    /* 9 */ update_data.status, // 状态
    /* 10 */ update_data.priority, // 优先级
    /* 11 */ update_data.estimated_income, // 预计收入
    /* 12 */ update_data.username_send, // 发送者
    /* 13 */ update_data.username_receive, // 接收者
    /* 14 */ update_data.username_delivery, // 配送者
    /* 15 */ update_data.telephone_send, // 发送者电话
    /* 16 */ update_data.telephone_receive, // 接收者电话
    /* 17 */ update_data.telephone_delivery, // 配送者电话
    /* 18 */ update_data.user_id_delivery_by, // 配送者id
    /* 19 */ update_data.user_id_created_by, // 谁创建的
  )
  .fetch_one(db)
  .await?;
  transaction.commit().await?;
  Ok(delivery_task)
}

pub async fn create_delivery_task(
  db: &sqlx::PgPool,
  task_data: DeliveryTaskCreateData, // 建议使用参数结构体
) -> Result<DeliveryTask> {
  // 开启事务
  let mut transaction = db.begin().await?;

  // 生成订单ID（UUID v4）
  let task_id = generate_task_id();

  // 获取当前时间（UTC）
  let create_time = beijing_time();

  let task = sqlx::query_as!(
    DeliveryTask,
    r#"
    INSERT INTO delivery_task (
            /* 1 */ id,
            /* 2 */ create_time,
            /* 3 */ arrived_time,
            /* 4 */ address_start_id,
            /* 5 */ address_end_id,
            /* 6 */ tag,
            /* 7 */ items,
            /* 8 */ note,
            /* 9 */ status,
            /* 10 */ priority,
            /* 11 */ estimated_income,
            /* 12 */ username_send,
            /* 13 */ username_receive,
            /* 14 */ username_delivery,
            /* 15 */ telephone_send,
            /* 16 */ telephone_receive,
            /* 17 */ telephone_delivery,
            /* 18 */ user_id_delivery_by,
            /* 19 */ user_id_created_by
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)
         RETURNING *;
        "#,
    /* 1 */ task_id,
    /* 2 */
    PrimitiveDateTime::new(create_time.date(), create_time.time()), // create_time 创建时间
    /* 3 */ None as Option<PrimitiveDateTime>, // arrived_time 送达时间
    /* 4 */ task_data.address_start_id, // 起始地址
    /* 5 */ task_data.address_end_id, // 结束地址
    /* 6 */ task_data.tag, // tag
    /* 7 */ task_data.items, // 物品
    /* 8 */ task_data.note, // 备注
    /* 9 */ "new", // 状态
    /* 10 */ task_data.priority.unwrap_or("default".to_string()), // 优先级
    /* 11 */ task_data.estimated_income, // 预计收入
    /* 12 */ task_data.username_send, // 发送者
    /* 13 */ task_data.username_receive, // 接收者
    /* 14 */ task_data.username_delivery, // 配送者
    /* 15 */ task_data.telephone_send, // 发送者电话
    /* 16 */ task_data.telephone_receive, // 接收者电话
    /* 17 */ task_data.telephone_delivery, // 配送者电话
    /* 18 */ task_data.user_id_delivery_by, // 配送者id
    /* 19 */ task_data.user_id_created_by, // 谁创建的
  )
  .fetch_one(&mut *transaction)
  .await?;

  // debug!("task create time: {}", task.create_time);
  transaction.commit().await?;
  Ok(task)
}

fn generate_task_id() -> String {
  format!("DT-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"))
}
