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
use crate::model::auth::UserType;
#[derive(Debug)]
enum Col {
    DeliveryPersonId(i64),
    Money(bigdecimal::BigDecimal),
    Status(String),
    CreatedAt(chrono::NaiveDateTime),
    UpdatedAt(chrono::NaiveDateTime),
    DeletedAt(chrono::NaiveDateTime),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryPerson {
    delivery_person_id: i64,
    money: bigdecimal::BigDecimal,
    status: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryPersonForCreate {
    delivery_person_id: Option<i64>,
    money: Option<bigdecimal::BigDecimal>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryPersonForUpdate {
    delivery_person_id: i64,
    money: Option<bigdecimal::BigDecimal>,
    status: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &DeliveryPersonForCreate,
) -> Result<DeliveryPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.delivery_person_id {
        None => {}
        Some(val) => {
            cols.push(Col::DeliveryPersonId(val.clone()));
            column_names.push(stringify!(delivery_person_id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
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
        "paotui.public.delivery_person", column_names.join(", "), placeholders
        .join(", "), stringify!(delivery_person_id, money, status, created_at,
        updated_at, deleted_at)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::DeliveryPersonId(val) => {
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
    data: &DeliveryPersonForUpdate,
) -> Result<DeliveryPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryPerson,
        "UPDATE paotui.public.delivery_person
         SET
           money = COALESCE($1, money),
           status = COALESCE($2, status),
           created_at = COALESCE($3, created_at),
           updated_at = COALESCE($4, updated_at),
           deleted_at = COALESCE($5, deleted_at)
         WHERE delivery_person_id = $6
         RETURNING delivery_person_id,money,status,created_at,updated_at,deleted_at
         ",
        data.money, data.status, data.created_at, data.updated_at, data.deleted_at, data
        .delivery_person_id
    )
        .fetch_one(executor)
        .await
}

pub async fn delete<'a, E>(
    executor: E,
    delivery_person_id: &i64,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        "DELETE FROM paotui.public.delivery_person WHERE delivery_person_id = $1",
        delivery_person_id
    )
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    delivery_person_id: i64,
) -> Result<DeliveryPerson, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryPerson,
        "SELECT * FROM paotui.public.delivery_person WHERE delivery_person_id = $1",
        delivery_person_id
    )
        .fetch_one(executor)
        .await
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStatusRequestBody {
    pub status: String,
}
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProfileStatus {
    Working,
    #[default]
    Resting,
}
impl ProfileStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "working" => Some(ProfileStatus::Working),
            "resting" => Some(ProfileStatus::Resting),
            _ => None,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClientProfile {
    pub user_type: UserType,
    pub user_phone: String,
    pub user_name: String,
    pub status: ProfileStatus,
}


#[instrument]
pub async fn query_delivery_person_by_delivery_person_id<'a, E>(executor: E, delivery_person_id: i64) -> Result<DeliveryPerson>where
  E: Executor<'a, Database = Postgres>  {
  let delivery_person = sqlx::query_as!(
    DeliveryPerson,
    "SELECT * FROM delivery_person WHERE delivery_person_id = $1",
    delivery_person_id
  )
    .fetch_one(executor)
    .await?;
  Ok(delivery_person)
}

pub async fn query_delivery_person_by_user_id<'a, E>(executor: E , user_id: i64) -> Result<DeliveryPerson>
where
  E: Executor<'a, Database = Postgres> {
  struct Result { id: i64 }
  let delivery_person = sqlx::query_as!(
    DeliveryPerson,
    r#"SELECT
        dp.delivery_person_id as delivery_person_id,
        dp.money as money,
        dp.status as status,
        dp.created_at as created_at,
        dp.updated_at as updated_at,
        dp.deleted_at as deleted_at
     FROM delivery_app_user dau
     LEFT JOIN public.delivery_person dp ON dau.delivery_person_id = dp.delivery_person_id
     WHERE user_id = $1;"#, user_id)
    .fetch_one(executor)
    .await?;
  Ok(delivery_person)
}

pub async fn create_delivery_person(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, delivery_person: CreateDeliveryPerson) -> Result<DeliveryPerson> {
  let now_time = now_time();
  let delivery_person = sqlx::query_as!(
    DeliveryPerson,
    r#"
        INSERT INTO delivery_person (
            money,
            status)
        VALUES ($1,$2)
        RETURNING *
        "#,
    delivery_person.money,
    delivery_person.status,
  )
    .fetch_one(&mut **transaction)
    .await?;
  Ok(delivery_person)
}


pub async fn update_delivery_person(
  transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
  delivery_person: UpdateDeliveryPerson,
) -> Result<DeliveryPerson> {
  let delivery_person = sqlx::query_as!(
    DeliveryPerson,
    r#"
        UPDATE delivery_person
        SET
           money = COALESCE($2, money),
           status = COALESCE($3, status),
           updated_at = COALESCE($4, updated_at)
        WHERE delivery_person_id = $1
        RETURNING *
        "#,
    delivery_person.delivery_person_id,
    delivery_person.money,
    delivery_person.status,
    now_time()
  )
    .fetch_one(&mut **transaction)
    .await?;
  Ok(delivery_person)
}

pub async fn delete_delivery_person_by_id(
  transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
  id: i64,
) -> Result<DeliveryPerson> {
  let delivery_person = sqlx::query_as!(
    DeliveryPerson,
    r#"
        DELETE FROM delivery_person
        WHERE delivery_person_id = $1
        RETURNING *
        "#,
    id
  )
    .fetch_one(&mut **transaction)
    .await?;
  Ok(delivery_person)
}