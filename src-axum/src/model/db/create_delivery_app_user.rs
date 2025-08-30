use crate::model::pg::delivery_app_user::DeliveryAppUser;

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