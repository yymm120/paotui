use serde::Serialize;
use sqlx::FromRow;
use time::PrimitiveDateTime;
use uuid::Uuid;
use crate::model::table::delivery_app_user::{CreateDeliveryAppUser, DeliveryAppUser, UpdateDeliveryAppUser};

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
  pub user_id: i64,
  pub username: String,
  pub password: String,
  pub telephone: String,
  pub created_at: PrimitiveDateTime,
  pub updated_at: PrimitiveDateTime,
  pub deleted_at: Option<PrimitiveDateTime>,
}


pub struct CreateUser {
  pub username: String,
  pub password: String,
  pub telephone: String,
}

impl From<CreateDeliveryAppUser> for CreateUser {
  fn from(delivery_user: CreateDeliveryAppUser) -> Self {
    Self {
      username: delivery_user.username,
      password: delivery_user.password,
      telephone: delivery_user.telephone,
    }
  }
}

pub struct UpdateUser {
  pub user_id: i64,
  pub username: Option<String>,
  pub password: Option<String>,
  pub telephone: Option<String>,
}

impl From<UpdateDeliveryAppUser> for UpdateUser {
fn from(delivery_user: UpdateDeliveryAppUser) -> Self {
  Self {
    user_id: delivery_user.user_id,
    username: delivery_user.username,
    password: delivery_user.password,
    telephone: delivery_user.telephone,
  }
}
}