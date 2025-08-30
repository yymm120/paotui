use sqlx::{Executor, Postgres};
use tracing::instrument;
use crate::model::table::delivery_task::DeliveryTask;
use crate::model::table::delivery_person::{CreateDeliveryPerson, DeliveryPerson, UpdateDeliveryPerson};
use crate::model::table::user::User;
use crate::query::delivery_app_user::query_delivery_app_user_by_user_id;
use crate::utils::db::Db;
use crate::query::error::Result;
use crate::utils::time::{beijing_time, now_time};

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