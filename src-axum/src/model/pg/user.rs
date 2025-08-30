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
    Username(String),
    Password(String),
    Telephone(String),
    UserType(String),
    CreatedAt(chrono::NaiveDateTime),
    UpdatedAt(chrono::NaiveDateTime),
    DeletedAt(chrono::NaiveDateTime),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct User {
    user_id: i64,
    username: String,
    password: String,
    telephone: String,
    user_type: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserForCreate {
    user_id: Option<i64>,
    username: String,
    password: String,
    telephone: String,
    user_type: String,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserForUpdate {
    user_id: i64,
    username: Option<String>,
    password: Option<String>,
    telephone: Option<String>,
    user_type: String,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    deleted_at: Option<chrono::NaiveDateTime>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &UserForCreate,
) -> Result<User, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.user_id {
        None => {}
        Some(val) => {
            cols.push(Col::UserId(val.clone()));
            column_names.push(stringify!(user_id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::Username(data.username.clone()));
    column_names.push(stringify!(username));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Password(data.password.clone()));
    column_names.push(stringify!(password));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Telephone(data.telephone.clone()));
    column_names.push(stringify!(telephone));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::UserType(data.user_type.clone()));
    column_names.push(stringify!(user_type));
    placeholders.push(format!("${}", column_names.len()));
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
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#, "paotui.public.user",
        column_names.join(", "), placeholders.join(", "), stringify!(user_id, username,
        password, telephone, user_type, created_at, updated_at, deleted_at)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::UserId(val) => {
                query = query.bind(val);
            }
            Col::Username(val) => {
                query = query.bind(val);
            }
            Col::Password(val) => {
                query = query.bind(val);
            }
            Col::Telephone(val) => {
                query = query.bind(val);
            }
            Col::UserType(val) => {
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
    data: &UserForUpdate,
) -> Result<User, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        User,
        "UPDATE paotui.public.user
         SET
           username = COALESCE($1, username),
           password = COALESCE($2, password),
           telephone = COALESCE($3, telephone),
           created_at = COALESCE($4, created_at),
           updated_at = COALESCE($5, updated_at),
           deleted_at = COALESCE($6, deleted_at)
         WHERE user_id = $7 AND user_type = $8
         RETURNING user_id,username,password,telephone,user_type,created_at,updated_at,deleted_at
         ",
        data.username, data.password, data.telephone, data.created_at, data.updated_at,
        data.deleted_at, data.user_id, data.user_type
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
        "DELETE FROM paotui.public.user WHERE user_id = $1 AND user_type = $2", user_id,
        user_type
    )
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    user_id: i64,
    user_type: String,
) -> Result<User, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        User, "SELECT * FROM paotui.public.user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_one(executor)
        .await
}
pub async fn query_by_telephone<'a, E>(
    executor: E,
    telephone: String,
) -> Result<Vec<User>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        User, "SELECT * FROM paotui.public.user WHERE telephone = $1", telephone
    )
        .fetch_all(executor)
        .await
}
pub async fn query_by_user_id_user_type<'a, E>(
    executor: E,
    user_id: i64,
    user_type: String,
) -> Result<Vec<User>, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        User, "SELECT * FROM paotui.public.user WHERE user_id = $1 AND user_type = $2",
        user_id, user_type
    )
        .fetch_all(executor)
        .await
}


pub enum Connection<'a> {
  Pool(Db),
  Transaction(&'a mut Transaction<'a, Postgres>),
}


fn query_by_user_phone(user_phone: String) -> Map<Postgres, fn(PgRow) -> Result<User>, PgArguments> {
  sqlx::query_as!(User,"SELECT * FROM public.user WHERE telephone = $1",user_phone)
}

fn query_by_user_id(user_id: i64) -> Map<Postgres, fn(PgRow) -> Result<User>, PgArguments> {
  sqlx::query_as!(User,"SELECT * FROM public.user WHERE user_id = $1",user_id)
}




// #[instrument]
pub async fn query_user_by_phone<'a, E>(
  executor: E,
  user_phone: String
) -> Result<User>
where
  E: Executor<'a, Database = Postgres>,
{
  let user = query_by_user_phone(user_phone.clone())
    .fetch_one(executor).await?;
  Ok(user)
}

// #[instrument]
pub async fn query_user_by_id(db: &Db, user_id: i64) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    "SELECT * FROM public.user WHERE user_id = $1",
    user_id
  )
    .fetch_one(db)
    .await?;
  Ok(user)
}

// #[instrument]
pub async fn query_user(db: &Db, user_id: i64, user_phone: String) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    "SELECT * FROM public.user WHERE user_id = $1 AND telephone = $2",
    user_id,
    user_phone
  )
    .fetch_one(db)
    .await?;
  Ok(user)
}


// #[instrument]

