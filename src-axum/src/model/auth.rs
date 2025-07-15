#![allow(unused)]

use chrono::Local;
use crate::model::jwt::Claims;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::table::user_table::UserTable;
use crate::utils::jwt::{generate_claims, generate_token, update_exp_and_iat};

/// ### 用户信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Profile {
  pub user_type: UserType,
  pub user_id: String,
  pub user_name: String,
  pub user_phone: String,
  pub token: String,
  pub claims: Claims,
  pub delivery_info: DeliveryInfo,
}

/// 运输信息
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DeliveryInfo {
  pub receive_name: String,
  pub receive_phone: String,
  pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseForLogin {
  pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBodyForLogin {
  pub user_phone: String,
  pub code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestBodyForCode {
  pub phone_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseForCode {
  pub code: String,
}

/// 有五种情况，对应三种中泰
/// ```markdown
/// - case 1: 用户不带token
/// - case 2: 用户带过期的token，
/// - case 3: 用户带不存在的user_id
/// - case 4: 用户带格式错误的token
/// - case 5: 正确的token, 存在的user_id
/// ```
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub enum UserType {
  // case 5
  VerifiedUser, // 已注册, 已登录的用户
  // case 1, 3, 4
  #[default]
  UnsignedUpUser, // 未注册的用户
  // case 2
  UnSignedInUser, // 已注册,未登录的用户
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserInfoForAutoLogin {
  pub phone_number: Option<String>,
  pub token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
  pub user_id: String,
  pub user_type: UserType,
  pub phone_number: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseForInitial {
  pub status: bool,
  pub user: User,
}

impl ResponseForInitial {
  pub fn login_success_response(user: User) -> ResponseForInitial {
    ResponseForInitial { status: true, user }
  }
  pub fn login_failed_response(user: User) -> ResponseForInitial {
    ResponseForInitial {
      status: false,
      user,
    }
  }
}

impl core::fmt::Display for UserType {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl Profile {
  pub fn new() -> Profile {
    Profile::default()
  }
  /// 创建一个`Profile`
  pub fn create_profile(
    user_type: UserType,
    user_item: Option<UserTable>
  ) -> Profile {
    let mut token = "".to_string();
    let mut claims = Claims::default();

    match user_type.clone() {
      UserType::VerifiedUser => {
        claims = generate_claims(UserType::VerifiedUser, user_item);
        token = generate_token(UserType::UnsignedUpUser, claims.clone()).unwrap_or("".to_string());
      }
      UserType::UnsignedUpUser => {
        claims = generate_claims(UserType::UnsignedUpUser, None);
        token = generate_token(UserType::UnsignedUpUser, claims.clone()).unwrap_or("".to_string());
      }
      UserType::UnSignedInUser => {
        claims = generate_claims(UserType::UnsignedUpUser, user_item);
        token = generate_token(UserType::UnsignedUpUser, claims.clone()).unwrap_or("".to_string());
      }
    }
    Profile {
      user_type,
      user_id: claims.sub.clone(),
      user_name: "".to_string(),
      user_phone: claims.phone.clone().into(),
      token: token.into(),
      claims,
      delivery_info: Default::default(),
    }
  }
  // pub fn update_profile(profile: Profile) ->
}
