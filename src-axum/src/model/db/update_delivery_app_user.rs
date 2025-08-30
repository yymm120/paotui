use sqlx::{Executor, Postgres};
use crate::model::pg::delivery_app_user::DeliveryAppUser;

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
