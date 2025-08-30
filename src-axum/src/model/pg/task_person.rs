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
    TaskPersonId(i64),
    CreatedAt(chrono::NaiveDateTime),
    UpdatedAt(chrono::NaiveDateTime),
    DeletedAt(chrono::NaiveDateTime),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct TaskPerson {
    task_person_id: i64,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskPersonForCreate {
    task_person_id: Option<i64>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskPersonForUpdate {
    task_person_id: i64,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &TaskPersonForCreate,
) -> Result<TaskPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.task_person_id {
        None => {}
        Some(val) => {
            cols.push(Col::TaskPersonId(val.clone()));
            column_names.push(stringify!(task_person_id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.created_at {
        None => {}
        Some(val) => {
            cols.push(Col::CreatedAt(val.clone()));
            column_names.push(stringify!(created_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.updated_at {
        None => {}
        Some(val) => {
            cols.push(Col::UpdatedAt(val.clone()));
            column_names.push(stringify!(updated_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.deleted_at {
        None => {}
        Some(val) => {
            cols.push(Col::DeletedAt(val.clone()));
            column_names.push(stringify!(deleted_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.task_person", column_names.join(", "), placeholders.join(", "),
        stringify!(task_person_id, created_at, updated_at, deleted_at)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::TaskPersonId(val) => {
                query = query.bind(val);
            }
            Col::CreatedAt(val) => {
                query = query.bind(val);
            }
            Col::UpdatedAt(val) => {
                query = query.bind(val);
            }
            Col::DeletedAt(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &TaskPersonForUpdate,
) -> Result<TaskPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskPerson,
        "UPDATE paotui.public.task_person
         SET
           created_at = COALESCE($1, created_at),
           updated_at = COALESCE($2, updated_at),
           deleted_at = COALESCE($3, deleted_at)
         WHERE task_person_id = $4
         RETURNING task_person_id,created_at,updated_at,deleted_at
         ",
        data.created_at, data.updated_at, data.deleted_at, data.task_person_id
    )
        .fetch_one(executor)
        .await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &TaskPersonForUpdate,
) -> Result<TaskPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskPerson,
        "UPDATE paotui.public.task_person
         SET
           created_at = COALESCE($1, created_at),
           updated_at = COALESCE($2, updated_at),
           deleted_at = COALESCE($3, deleted_at)
         WHERE task_person_id = $4
         RETURNING task_person_id,created_at,updated_at,deleted_at
         ",
        data.created_at, data.updated_at, data.deleted_at, data.task_person_id
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(executor: E, task_person_id: &i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "DELETE FROM paotui.public.task_person WHERE task_person_id = $1", task_person_id
    )
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    task_person_id: i64,
) -> Result<TaskPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskPerson, "SELECT * FROM paotui.public.task_person WHERE task_person_id = $1",
        task_person_id
    )
        .fetch_one(executor)
        .await
}
