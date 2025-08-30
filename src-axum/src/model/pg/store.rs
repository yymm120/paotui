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
    StoreId(i64),
    StoreName(String),
    Password(String),
    Address(i64),
    Telephone(String),
    Money(bigdecimal::BigDecimal),
    Status(String),
    CreatedAt(chrono::NaiveDateTime),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct Store {
    store_id: i64,
    store_name: String,
    password: String,
    address: i64,
    telephone: String,
    money: bigdecimal::BigDecimal,
    status: String,
    created_at: chrono::NaiveDateTime,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreForCreate {
    store_id: Option<i64>,
    store_name: String,
    password: String,
    address: i64,
    telephone: String,
    money: Option<bigdecimal::BigDecimal>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StoreForUpdate {
    store_id: i64,
    store_name: Option<String>,
    password: Option<String>,
    address: Option<i64>,
    telephone: Option<String>,
    money: Option<bigdecimal::BigDecimal>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &StoreForCreate,
) -> Result<Store, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.store_id {
        None => {}
        Some(val) => {
            cols.push(Col::StoreId(val.clone()));
            column_names.push(stringify!(store_id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::StoreName(data.store_name.clone()));
    column_names.push(stringify!(store_name));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Password(data.password.clone()));
    column_names.push(stringify!(password));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Address(data.address.clone()));
    column_names.push(stringify!(address));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Telephone(data.telephone.clone()));
    column_names.push(stringify!(telephone));
    placeholders.push(format!("${}", column_names.len()));
    match &data.money {
        None => {}
        Some(val) => {
            cols.push(Col::Money(val.clone()));
            column_names.push(stringify!(money));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.status {
        None => {}
        Some(val) => {
            cols.push(Col::Status(val.clone()));
            column_names.push(stringify!(status));
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
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#, "paotui.public.store",
        column_names.join(", "), placeholders.join(", "), stringify!(store_id,
        store_name, password, address, telephone, money, status, created_at)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::StoreId(val) => {
                query = query.bind(val);
            }
            Col::StoreName(val) => {
                query = query.bind(val);
            }
            Col::Password(val) => {
                query = query.bind(val);
            }
            Col::Address(val) => {
                query = query.bind(val);
            }
            Col::Telephone(val) => {
                query = query.bind(val);
            }
            Col::Money(val) => {
                query = query.bind(val);
            }
            Col::Status(val) => {
                query = query.bind(val);
            }
            Col::CreatedAt(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &StoreForUpdate,
) -> Result<Store, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Store,
        "UPDATE paotui.public.store
         SET
           store_name = COALESCE($1, store_name),
           password = COALESCE($2, password),
           address = COALESCE($3, address),
           telephone = COALESCE($4, telephone),
           money = COALESCE($5, money),
           status = COALESCE($6, status),
           created_at = COALESCE($7, created_at)
         WHERE store_id = $8
         RETURNING store_id,store_name,password,address,telephone,money,status,created_at
         ",
        data.store_name, data.password, data.address, data.telephone, data.money, data
        .status, data.created_at, data.store_id
    )
        .fetch_one(executor)
        .await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &StoreForUpdate,
) -> Result<Store, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Store,
        "UPDATE paotui.public.store
         SET
           store_name = COALESCE($1, store_name),
           password = COALESCE($2, password),
           address = COALESCE($3, address),
           telephone = COALESCE($4, telephone),
           money = COALESCE($5, money),
           status = COALESCE($6, status),
           created_at = COALESCE($7, created_at)
         WHERE store_id = $8
         RETURNING store_id,store_name,password,address,telephone,money,status,created_at
         ",
        data.store_name, data.password, data.address, data.telephone, data.money, data
        .status, data.created_at, data.store_id
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(executor: E, store_id: &i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM paotui.public.store WHERE store_id = $1", store_id)
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    store_id: i64,
) -> Result<Store, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Store, "SELECT * FROM paotui.public.store WHERE store_id = $1", store_id
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_telephone<'a, E>(
    executor: E,
    telephone: String,
) -> Result<Store, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Store, "SELECT * FROM paotui.public.store WHERE telephone = $1", telephone
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_status<'a, E>(
    executor: E,
    status: String,
) -> Result<Vec<Store>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(Store, "SELECT * FROM paotui.public.store WHERE status = $1", status)
        .fetch_all(executor)
        .await
}
