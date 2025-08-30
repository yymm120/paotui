//! =====================
//!
//! Meta Information:
//! - name: 'paotui.public.delivery_task'
//! - sync: true
//! - Enum: [Col]
//! - Struct: [DeliveryTask], [DeliveryTaskForCreate], [DeliveryTaskForUpdate]
//! - methods: [insert], [update], [delete], [retrieve]
//! - oid: #40238#
//!
//! Note: Please do not delete comments, the code generation tool relies on comments.
//!
//! Examples:
//! ```rust
//! fn main () {
//!   let transaction = pool.begin();
//!   let data = DeliveryForCreate::default();
//!   model::insert(&mut *transaction, &data);
//!   transaction.commit();
//! }
//! ```
//!
//! =====================
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, PgPool, Postgres};
#[derive(Debug)]
enum Col {
    Id(String),
    Tag(String),
    Items(String),
    Note(String),
    Status(String),
    Priority(String),
    EstimatedIncome(String),
    UsernameSend(String),
    UsernameReceive(String),
    UsernameDelivery(String),
    AddressStartId(i64),
    AddressEndId(i64),
    UserIdCreatedBy(i64),
    TelephoneSend(String),
    TelephoneReceive(String),
    TelephoneDelivery(String),
    UserIdDeliveryBy(i64),
    CreateTime(chrono::NaiveDateTime),
    ArrivedTime(chrono::NaiveDateTime),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryTask {
    id: String,
    tag: Option<String>,
    items: Option<String>,
    note: Option<String>,
    status: String,
    priority: String,
    estimated_income: String,
    username_send: String,
    username_receive: String,
    username_delivery: Option<String>,
    address_start_id: i64,
    address_end_id: i64,
    user_id_created_by: i64,
    telephone_send: String,
    telephone_receive: String,
    telephone_delivery: Option<String>,
    user_id_delivery_by: Option<i64>,
    create_time: chrono::NaiveDateTime,
    arrived_time: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryTaskForCreate {
    id: String,
    tag: Option<String>,
    items: Option<String>,
    note: Option<String>,
    status: String,
    priority: Option<String>,
    estimated_income: String,
    username_send: String,
    username_receive: String,
    username_delivery: Option<String>,
    address_start_id: i64,
    address_end_id: i64,
    user_id_created_by: i64,
    telephone_send: String,
    telephone_receive: String,
    telephone_delivery: Option<String>,
    user_id_delivery_by: Option<i64>,
    create_time: chrono::NaiveDateTime,
    arrived_time: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryTaskForUpdate {
    id: String,
    tag: Option<String>,
    items: Option<String>,
    note: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    estimated_income: Option<String>,
    username_send: Option<String>,
    username_receive: Option<String>,
    username_delivery: Option<String>,
    address_start_id: Option<i64>,
    address_end_id: Option<i64>,
    user_id_created_by: Option<i64>,
    telephone_send: Option<String>,
    telephone_receive: Option<String>,
    telephone_delivery: Option<String>,
    user_id_delivery_by: Option<i64>,
    create_time: Option<chrono::NaiveDateTime>,
    arrived_time: Option<chrono::NaiveDateTime>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &DeliveryTaskForCreate,
) -> Result<DeliveryTask, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    cols.push(Col::Id(data.id.clone()));
    column_names.push(stringify!(id));
    placeholders.push(format!("${}", column_names.len()));
    match &data.tag {
        None => {}
        Some(val) => {
            cols.push(Col::Tag(val.clone()));
            column_names.push(stringify!(tag));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.items {
        None => {}
        Some(val) => {
            cols.push(Col::Items(val.clone()));
            column_names.push(stringify!(items));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.note {
        None => {}
        Some(val) => {
            cols.push(Col::Note(val.clone()));
            column_names.push(stringify!(note));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::Status(data.status.clone()));
    column_names.push(stringify!(status));
    placeholders.push(format!("${}", column_names.len()));
    match &data.priority {
        None => {}
        Some(val) => {
            cols.push(Col::Priority(val.clone()));
            column_names.push(stringify!(priority));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::EstimatedIncome(data.estimated_income.clone()));
    column_names.push(stringify!(estimated_income));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UsernameSend(data.username_send.clone()));
    column_names.push(stringify!(username_send));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UsernameReceive(data.username_receive.clone()));
    column_names.push(stringify!(username_receive));
    placeholders.push(format!("${}", column_names.len()));
    match &data.username_delivery {
        None => {}
        Some(val) => {
            cols.push(Col::UsernameDelivery(val.clone()));
            column_names.push(stringify!(username_delivery));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::AddressStartId(data.address_start_id.clone()));
    column_names.push(stringify!(address_start_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::AddressEndId(data.address_end_id.clone()));
    column_names.push(stringify!(address_end_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UserIdCreatedBy(data.user_id_created_by.clone()));
    column_names.push(stringify!(user_id_created_by));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::TelephoneSend(data.telephone_send.clone()));
    column_names.push(stringify!(telephone_send));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::TelephoneReceive(data.telephone_receive.clone()));
    column_names.push(stringify!(telephone_receive));
    placeholders.push(format!("${}", column_names.len()));
    match &data.telephone_delivery {
        None => {}
        Some(val) => {
            cols.push(Col::TelephoneDelivery(val.clone()));
            column_names.push(stringify!(telephone_delivery));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.user_id_delivery_by {
        None => {}
        Some(val) => {
            cols.push(Col::UserIdDeliveryBy(val.clone()));
            column_names.push(stringify!(user_id_delivery_by));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::CreateTime(data.create_time.clone()));
    column_names.push(stringify!(create_time));
    placeholders.push(format!("${}", column_names.len()));
    match &data.arrived_time {
        None => {}
        Some(val) => {
            cols.push(Col::ArrivedTime(val.clone()));
            column_names.push(stringify!(arrived_time));
            placeholders.push(format!("${}", column_names.len()));
        }
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.delivery_task", column_names.join(", "), placeholders.join(", "),
        stringify!(id, tag, items, note, status, priority, estimated_income,
        username_send, username_receive, username_delivery, address_start_id,
        address_end_id, user_id_created_by, telephone_send, telephone_receive,
        telephone_delivery, user_id_delivery_by, create_time, arrived_time)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::Id(val) => {
                query = query.bind(val);
            }
            Col::Tag(val) => {
                query = query.bind(val);
            }
            Col::Items(val) => {
                query = query.bind(val);
            }
            Col::Note(val) => {
                query = query.bind(val);
            }
            Col::Status(val) => {
                query = query.bind(val);
            }
            Col::Priority(val) => {
                query = query.bind(val);
            }
            Col::EstimatedIncome(val) => {
                query = query.bind(val);
            }
            Col::UsernameSend(val) => {
                query = query.bind(val);
            }
            Col::UsernameReceive(val) => {
                query = query.bind(val);
            }
            Col::UsernameDelivery(val) => {
                query = query.bind(val);
            }
            Col::AddressStartId(val) => {
                query = query.bind(val);
            }
            Col::AddressEndId(val) => {
                query = query.bind(val);
            }
            Col::UserIdCreatedBy(val) => {
                query = query.bind(val);
            }
            Col::TelephoneSend(val) => {
                query = query.bind(val);
            }
            Col::TelephoneReceive(val) => {
                query = query.bind(val);
            }
            Col::TelephoneDelivery(val) => {
                query = query.bind(val);
            }
            Col::UserIdDeliveryBy(val) => {
                query = query.bind(val);
            }
            Col::CreateTime(val) => {
                query = query.bind(val);
            }
            Col::ArrivedTime(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &DeliveryTaskForUpdate,
) -> Result<DeliveryTask, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryTask,
        "UPDATE paotui.public.delivery_task
         SET
           tag = COALESCE($1, tag),
           items = COALESCE($2, items),
           note = COALESCE($3, note),
           status = COALESCE($4, status),
           priority = COALESCE($5, priority),
           estimated_income = COALESCE($6, estimated_income),
           username_send = COALESCE($7, username_send),
           username_receive = COALESCE($8, username_receive),
           username_delivery = COALESCE($9, username_delivery),
           address_start_id = COALESCE($10, address_start_id),
           address_end_id = COALESCE($11, address_end_id),
           user_id_created_by = COALESCE($12, user_id_created_by),
           telephone_send = COALESCE($13, telephone_send),
           telephone_receive = COALESCE($14, telephone_receive),
           telephone_delivery = COALESCE($15, telephone_delivery),
           user_id_delivery_by = COALESCE($16, user_id_delivery_by),
           create_time = COALESCE($17, create_time),
           arrived_time = COALESCE($18, arrived_time)
         WHERE id = $19
         RETURNING id,tag,items,note,status,priority,estimated_income,username_send,username_receive,username_delivery,address_start_id,address_end_id,user_id_created_by,telephone_send,telephone_receive,telephone_delivery,user_id_delivery_by,create_time,arrived_time
         ",
        data.tag, data.items, data.note, data.status, data.priority, data
        .estimated_income, data.username_send, data.username_receive, data
        .username_delivery, data.address_start_id, data.address_end_id, data
        .user_id_created_by, data.telephone_send, data.telephone_receive, data
        .telephone_delivery, data.user_id_delivery_by, data.create_time, data
        .arrived_time, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &DeliveryTaskForUpdate,
) -> Result<DeliveryTask, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryTask,
        "UPDATE paotui.public.delivery_task
         SET
           tag = COALESCE($1, tag),
           items = COALESCE($2, items),
           note = COALESCE($3, note),
           status = COALESCE($4, status),
           priority = COALESCE($5, priority),
           estimated_income = COALESCE($6, estimated_income),
           username_send = COALESCE($7, username_send),
           username_receive = COALESCE($8, username_receive),
           username_delivery = COALESCE($9, username_delivery),
           address_start_id = COALESCE($10, address_start_id),
           address_end_id = COALESCE($11, address_end_id),
           user_id_created_by = COALESCE($12, user_id_created_by),
           telephone_send = COALESCE($13, telephone_send),
           telephone_receive = COALESCE($14, telephone_receive),
           telephone_delivery = COALESCE($15, telephone_delivery),
           user_id_delivery_by = COALESCE($16, user_id_delivery_by),
           create_time = COALESCE($17, create_time),
           arrived_time = COALESCE($18, arrived_time)
         WHERE id = $19
         RETURNING id,tag,items,note,status,priority,estimated_income,username_send,username_receive,username_delivery,address_start_id,address_end_id,user_id_created_by,telephone_send,telephone_receive,telephone_delivery,user_id_delivery_by,create_time,arrived_time
         ",
        data.tag, data.items, data.note, data.status, data.priority, data
        .estimated_income, data.username_send, data.username_receive, data
        .username_delivery, data.address_start_id, data.address_end_id, data
        .user_id_created_by, data.telephone_send, data.telephone_receive, data
        .telephone_delivery, data.user_id_delivery_by, data.create_time, data
        .arrived_time, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(executor: E, id: &String) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM paotui.public.delivery_task WHERE id = $1", id)
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    id: String,
) -> Result<DeliveryTask, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryTask, "SELECT * FROM paotui.public.delivery_task WHERE id = $1", id
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_user_id_created_by<'a, E>(
    executor: E,
    user_id_created_by: i64,
) -> Result<Vec<DeliveryTask>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryTask,
        "SELECT * FROM paotui.public.delivery_task WHERE user_id_created_by = $1",
        user_id_created_by
    )
        .fetch_all(executor)
        .await
}
pub async fn query_by_user_id_delivery_by<'a, E>(
    executor: E,
    user_id_delivery_by: Option<i64>,
) -> Result<Vec<DeliveryTask>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryTask,
        "SELECT * FROM paotui.public.delivery_task WHERE user_id_delivery_by = $1",
        user_id_delivery_by
    )
        .fetch_all(executor)
        .await
}


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

