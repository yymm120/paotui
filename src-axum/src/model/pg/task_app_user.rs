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
    UserId(i64),
    TaskPersonId(i64),
    UserType(String),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct TaskAppUser {
    user_id: i64,
    task_person_id: i64,
    user_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskAppUserForCreate {
    user_id: i64,
    task_person_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskAppUserForUpdate {
    user_id: i64,
    task_person_id: Option<i64>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &TaskAppUserForCreate,
) -> Result<TaskAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    cols.push(Col::UserId(data.user_id.clone()));
    column_names.push(stringify!(user_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::TaskPersonId(data.task_person_id.clone()));
    column_names.push(stringify!(task_person_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UserType(data.user_type.clone()));
    column_names.push(stringify!(user_type));
    placeholders.push(format!("${}", column_names.len()));
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.task_app_user", column_names.join(", "), placeholders.join(", "),
        stringify!(user_id, task_person_id, user_type)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::UserId(val) => {
                query = query.bind(val);
            }
            Col::TaskPersonId(val) => {
                query = query.bind(val);
            }
            Col::UserType(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}

pub async fn update_safe<'a, E>(
    executor: E,
    data: &TaskAppUserForUpdate,
) -> Result<TaskAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAppUser,
        "UPDATE paotui.public.task_app_user
         SET
           task_person_id = COALESCE($1, task_person_id)
         WHERE user_id = $2 AND user_type = $3
         RETURNING user_id,task_person_id,user_type
         ",
        data.task_person_id, data.user_id, data.user_type
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(
    executor: E,
    user_id: &i64,
    user_type: &String,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "DELETE FROM paotui.public.task_app_user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    user_id: i64,
    user_type: String,
) -> Result<TaskAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAppUser,
        "SELECT * FROM paotui.public.task_app_user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_user_id<'a, E>(
    executor: E,
    user_id: i64,
) -> Result<TaskAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAppUser, "SELECT * FROM paotui.public.task_app_user WHERE user_id = $1",
        user_id
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_user_id_user_type<'a, E>(
    executor: E,
    user_id: i64,
    user_type: String,
) -> Result<Vec<TaskAppUser>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAppUser,
        "SELECT * FROM paotui.public.task_app_user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_all(executor)
        .await
}
