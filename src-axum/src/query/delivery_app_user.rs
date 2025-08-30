use crate::model::table::user::{CreateUser, UpdateUser, User};
use crate::query::error::{QueryError, Result};
use crate::utils::db::Db;
use std::str::FromStr;
use sqlx::{Error, Executor, Postgres, Row, Transaction};
use tracing::{debug, error, info, instrument};
use crate::model::table::delivery_app_user::{CreateDeliveryAppUser, DeliveryAppUser, UpdateDeliveryAppUser};
use crate::model::table::delivery_person::{CreateDeliveryPerson, DeliveryPerson, UpdateDeliveryPerson};
use crate::query::delivery_person::{create_delivery_person, delete_delivery_person_by_id, query_delivery_person_by_user_id, update_delivery_person};
use crate::query::user::{create_user, delete_user, query_user_by_phone, update_user, Connection};
use crate::utils::time::now_time;

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

struct AddressOperation<'a, E>(E)
where
  E: Executor<'a, Database = Postgres> + 'a;

// #[instrument]
pub async fn create_delivery_app_user(db: &Db, delivery_user: CreateDeliveryAppUser) -> Result<DeliveryAppUser> {
  let mut transaction = db.begin().await?;

  let user = query_user_by_phone(&mut transaction, delivery_user.telephone.clone())
    .await
    .unwrap_or(create_user(&mut transaction, CreateUser::from(delivery_user.clone())).await?);

  let delivery_person = query_delivery_person_by_user_id(&mut **(&mut transaction), user.user_id)
    .await
    .unwrap_or(create_delivery_person(&mut transaction, CreateDeliveryPerson::from(delivery_user)).await?);

  sqlx::query_as!(
    DeliveryAppUser,
    r#"
        INSERT INTO public.delivery_app_user (user_id, delivery_person_id)
        VALUES ($1, $2);
        "#,
    user.user_id,
    delivery_person.delivery_person_id,
  )
    .execute(&mut **(&mut transaction))
    .await?;
  transaction.commit().await?;
  Ok(DeliveryAppUser::from((user, delivery_person)))
}


pub async fn delete_delivery_app_user(db: &Db, user_id: i64) -> Result<DeliveryAppUser> {
  let mut transaction = db.begin().await?;
  let delivery_person = query_delivery_person_by_user_id(&mut **(&mut transaction), user_id).await?;

  let user = delete_user(&mut transaction, user_id).await?;
  let delivery_person = delete_delivery_person_by_id(&mut transaction, delivery_person.delivery_person_id).await?;

  sqlx::query_as!(None, r#"
    DELETE FROM public.delivery_app_user WHERE user_id = $1 AND delivery_person_id = $2;
  "#,
  user_id,
    delivery_person.delivery_person_id
  ).execute(db)
    .await?;
  transaction.commit().await?;

  Ok(DeliveryAppUser::from((user, delivery_person)))
}
pub trait IntoExecutor {
  fn exe<'a, E>(self: &mut Transaction<Postgres>) -> E where E: Executor<'a, Database = Postgres>;
}

impl  IntoExecutor for Transaction<Postgres> {
  fn exe<'a, E>(mut self: &mut Transaction<Postgres>) -> E
  where
    E: Executor<'a, Database=Postgres>
  {
    &mut **(self)
  }
}

pub async fn update_delivery_app_user(
  db: &sqlx::PgPool,
  update_data: UpdateDeliveryAppUser,
) -> Result<DeliveryAppUser> {
  let mut transaction: Transaction<Postgres> = db.begin().await?;
  fn a<'a, E>(mut t: Transaction<Postgres>) -> E where
    E: Executor<'a, Database = Postgres>{
    &mut **(&mut t)
  }
  let delivery_person = query_delivery_person_by_user_id(transaction.exe(), update_data.user_id).await?;
  let user = update_user(&mut transaction, UpdateUser::from(update_data.clone())).await?;
  let delivery_person = update_delivery_person(&mut transaction, UpdateDeliveryPerson::from((delivery_person.delivery_person_id, update_data))).await?;
  transaction.commit().await?;
  Ok(DeliveryAppUser::from((user, delivery_person)))
}


