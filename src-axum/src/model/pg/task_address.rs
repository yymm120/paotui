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
    Id(i64),
    AddressId(f64),
    AddressDetail(String),
    City(String),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct TaskAddress {
    id: i64,
    address_id: f64,
    address_detail: Option<String>,
    city: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskAddressForCreate {
    id: Option<i64>,
    address_id: f64,
    address_detail: Option<String>,
    city: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskAddressForUpdate {
    id: i64,
    address_id: Option<f64>,
    address_detail: Option<String>,
    city: Option<String>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &TaskAddressForCreate,
) -> Result<TaskAddress, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.id {
        None => {}
        Some(val) => {
            cols.push(Col::Id(val.clone()));
            column_names.push(stringify!(id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::AddressId(data.address_id.clone()));
    column_names.push(stringify!(address_id));
    placeholders.push(format!("${}", column_names.len()));
    match &data.address_detail {
        None => {}
        Some(val) => {
            cols.push(Col::AddressDetail(val.clone()));
            column_names.push(stringify!(address_detail));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.city {
        None => {}
        Some(val) => {
            cols.push(Col::City(val.clone()));
            column_names.push(stringify!(city));
            placeholders.push(format!("${}", column_names.len()));
        }
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.task_address", column_names.join(", "), placeholders.join(", "),
        stringify!(id, address_id, address_detail, city)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::Id(val) => {
                query = query.bind(val);
            }
            Col::AddressId(val) => {
                query = query.bind(val);
            }
            Col::AddressDetail(val) => {
                query = query.bind(val);
            }
            Col::City(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &TaskAddressForUpdate,
) -> Result<TaskAddress, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAddress,
        "UPDATE paotui.public.task_address
         SET
           address_id = COALESCE($1, address_id),
           address_detail = COALESCE($2, address_detail),
           city = COALESCE($3, city)
         WHERE id = $4
         RETURNING id,address_id,address_detail,city
         ",
        data.address_id, data.address_detail, data.city, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &TaskAddressForUpdate,
) -> Result<TaskAddress, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAddress,
        "UPDATE paotui.public.task_address
         SET
           address_id = COALESCE($1, address_id),
           address_detail = COALESCE($2, address_detail),
           city = COALESCE($3, city)
         WHERE id = $4
         RETURNING id,address_id,address_detail,city
         ",
        data.address_id, data.address_detail, data.city, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(executor: E, id: &i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM paotui.public.task_address WHERE id = $1", id)
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    id: i64,
) -> Result<TaskAddress, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        TaskAddress, "SELECT * FROM paotui.public.task_address WHERE id = $1", id
    )
        .fetch_one(executor)
        .await
}
