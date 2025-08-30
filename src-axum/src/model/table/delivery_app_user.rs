use fake::Dummy;
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::FromRow;
use time::PrimitiveDateTime;
use crate::model::table::delivery_person::{CreateDeliveryPerson, DeliveryPerson};
use crate::model::table::user::User;

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct DeliveryAppUser {
  pub user_id: i64,
  pub username: String,
  pub telephone: String,
  pub money: Decimal,
  pub status: String,
}

impl From<(User, DeliveryPerson)> for DeliveryAppUser {
  fn from((user, delivery_person): (User, DeliveryPerson)) -> Self {
    Self {
      user_id: user.user_id,
      username: user.username,
      money: delivery_person.money,
      status: delivery_person.status,
      telephone: user.telephone,
    }
  }
}




#[derive(Clone, FromRow, Debug, Serialize)]
pub struct CreateDeliveryAppUser {
  pub username: String,
  pub password: String,
  pub telephone: String,
  pub money: Decimal,
  pub status: String,
}

#[derive(Clone, FromRow, Debug, Serialize, Default)]
pub struct UpdateDeliveryAppUser {
  pub user_id: i64,
  pub username: Option<String>,
  pub password: Option<String>,
  pub telephone: Option<String>,
  pub money: Option<Decimal>,
  pub status: Option<String>,
}

impl UpdateDeliveryAppUser {
  pub fn change_status(user_id: i64, status: String) -> Self {
    let mut update_data = UpdateDeliveryAppUser::default();
    update_data.user_id = user_id;
    update_data.status = Some(status);
    update_data
  }
}