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
    CustomerPersonId(i64),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct CustomerPerson {
    customer_person_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustomerPersonForCreate {
    customer_person_id: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CustomerPersonForUpdate {
    customer_person_id: i64,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &CustomerPersonForCreate,
) -> Result<CustomerPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.customer_person_id {
        None => {}
        Some(val) => {
            cols.push(Col::CustomerPersonId(val.clone()));
            column_names.push(stringify!(customer_person_id));
            placeholders.push(format!("${}", column_names.len()));
        }
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.customer_person", column_names.join(", "), placeholders
        .join(", "), stringify!(customer_person_id)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::CustomerPersonId(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn delete<'a, E>(
    executor: E,
    customer_person_id: &i64,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "DELETE FROM paotui.public.customer_person WHERE customer_person_id = $1",
        customer_person_id
    )
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    customer_person_id: i64,
) -> Result<CustomerPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        CustomerPerson,
        "SELECT * FROM paotui.public.customer_person WHERE customer_person_id = $1",
        customer_person_id
    )
        .fetch_one(executor)
        .await
}
