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
    DeliveryPersonId(i64),
    UserId(i64),
    UserType(String),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryAppUser {
    delivery_person_id: i64,
    user_id: i64,
    user_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryAppUserForCreate {
    delivery_person_id: i64,
    user_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeliveryAppUserForUpdate {
    delivery_person_id: Option<i64>,
    user_id: i64,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &DeliveryAppUserForCreate,
) -> Result<DeliveryAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    cols.push(Col::DeliveryPersonId(data.delivery_person_id.clone()));
    column_names.push(stringify!(delivery_person_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UserId(data.user_id.clone()));
    column_names.push(stringify!(user_id));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UserType(data.user_type.clone()));
    column_names.push(stringify!(user_type));
    placeholders.push(format!("${}", column_names.len()));
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#,
        "paotui.public.delivery_app_user", column_names.join(", "), placeholders
        .join(", "), stringify!(delivery_person_id, user_id, user_type)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::DeliveryPersonId(val) => {
                query = query.bind(val);
            }
            Col::UserId(val) => {
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
    data: &DeliveryAppUserForUpdate,
) -> Result<DeliveryAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryAppUser,
        "UPDATE paotui.public.delivery_app_user
         SET
           delivery_person_id = COALESCE($1, delivery_person_id)
         WHERE user_id = $2 AND user_type = $3
         RETURNING delivery_person_id,user_id,user_type
         ",
        data.delivery_person_id, data.user_id, data.user_type
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
        "DELETE FROM paotui.public.delivery_app_user WHERE user_id = $1 AND user_type = $2",
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
) -> Result<DeliveryAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryAppUser,
        "SELECT * FROM paotui.public.delivery_app_user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_user_id<'a, E>(
    executor: E,
    user_id: i64,
) -> Result<DeliveryAppUser, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryAppUser,
        "SELECT * FROM paotui.public.delivery_app_user WHERE user_id = $1", user_id
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_user_id_user_type<'a, E>(
    executor: E,
    user_id: i64,
    user_type: String,
) -> Result<Vec<DeliveryAppUser>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        DeliveryAppUser,
        "SELECT * FROM paotui.public.delivery_app_user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_all(executor)
        .await
}



// #[instrument]
pub async fn query_delivery_app_user_by_user_id(db: &Db, user_id: i64) -> Result<DeliveryAppUser> {
  let user = sqlx::query_as!(
    DeliveryAppUser,
    r#"
    SELECT
        u.user_id AS user_id,
        u.username AS username,
        u.telephone AS telephone,
        dp.money AS money,
        dp.status AS status
    FROM public.delivery_app_user du
    LEFT JOIN public.user u ON u.user_id = du.user_id
    LEFT JOIN public.delivery_person dp ON du.delivery_person_id = dp.delivery_person_id
    WHERE du.user_id = $1
    ;"#,
    user_id
  )
    .fetch_one(db)
    .await?;

  Ok(user)
}

// #[instrument]
pub async fn query_delivery_app_user_by_phone(db: &Db, telephone: String) -> Result<DeliveryAppUser> {
  let user = sqlx::query_as!(
    DeliveryAppUser,
    r#"
    SELECT
        u.user_id AS user_id,
        u.username AS username,
        u.telephone AS telephone,
        dp.money AS money,
        dp.status AS status
    FROM public.delivery_app_user du
    LEFT JOIN public.user u ON u.user_id = du.user_id
    LEFT JOIN public.delivery_person dp ON du.delivery_person_id = dp.delivery_person_id
    WHERE u.telephone = $1
    ;"#,
    telephone
  )
    .fetch_one(db)
    .await?;

  Ok(user)
}



// pub trait IntoExecutor {
//   fn exe<'a, E>(self: &mut Transaction<Postgres>) -> E where E: Executor<'a, Database = Postgres>;
// }
//
// impl  IntoExecutor for Transaction<Postgres> {
//   fn exe<'a, E>(mut self: &mut Transaction<Postgres>) -> E
//   where
//     E: Executor<'a, Database=Postgres>
//   {
//     &mut **(self)
//   }
// }


