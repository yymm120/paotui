use crate::model::pg::delivery_app_user::DeliveryAppUser;

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