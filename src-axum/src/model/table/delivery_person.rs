use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::Display;
use time::PrimitiveDateTime;
use crate::model::auth::UserType;
use crate::model::table::delivery_app_user::{CreateDeliveryAppUser, UpdateDeliveryAppUser};
use crate::model::table::user::{CreateUser, UpdateUser, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPerson {
  pub delivery_person_id: i64,
  pub money: Decimal,
  pub status: String,
  pub created_at: PrimitiveDateTime,
  pub updated_at: PrimitiveDateTime,
  pub deleted_at: Option<PrimitiveDateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeliveryPerson {
  pub money: Decimal,
  pub status: String,
}
impl From<CreateDeliveryAppUser> for CreateDeliveryPerson {
  fn from(delivery_user: CreateDeliveryAppUser) -> Self {
    Self {
      money: delivery_user.money,
      status: delivery_user.status,
    }
  }
}

// 用于更新用户的DTO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDeliveryPerson {
  pub delivery_person_id: i64,
  pub money: Option<Decimal>,
  pub status: Option<String>,
}
impl From<(i64, UpdateDeliveryAppUser)> for UpdateDeliveryPerson {
  fn from((delivery_person_id, delivery_user): (i64, UpdateDeliveryAppUser)) -> Self {
    Self {
      delivery_person_id,
      money: delivery_user.money,
      status: delivery_user.status,
    }
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateStatusRequestBody {
  pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, Display)]
#[serde(rename_all = "lowercase")]
pub enum ProfileStatus {
  Working,
  #[default]
  Resting
}

impl ProfileStatus {
  pub fn from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
      "working" => Some(ProfileStatus::Working),
      "resting" => Some(ProfileStatus::Resting),
      _ => None, // 未知状态
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