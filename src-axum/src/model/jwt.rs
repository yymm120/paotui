use crate::model::error::Result;
use crate::model::table::user_table::UserTable;
use crate::utils::db::Db;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Claims {
  pub sub: String,   // 标准字段: 用户ID
  pub exp: i64,      // 标准字段: 过期时间
  pub iat: i64,      // 签发时间: 签发时间
  pub phone: String, // 自定义字段: 用户手机号
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[validate(context = UserTable)]
#[validate(schema(
  function = "validate_claims",
  skip_on_field_errors = false,
  use_context
))]
pub struct ValidClaims {
  #[validate(length(equal = 36, message = "invalid uuid!"))]
  pub sub: String, // 标准字段: 用户ID
  pub exp: i64, // 标准字段: 过期时间
  pub iat: i64, // 签发时间: 签发时间
  #[validate(length(min = 11, message = "手机号码错误!"))]
  pub phone: String, // 自定义字段: 用户手机号
}

#[derive(Serialize, Deserialize)]
pub struct EncodeData {
  claims: Claims,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeData {
  pub token: String,
  pub validate_aud: Option<bool>,
  pub validate_exp: Option<bool>,
  pub validate_nbf: Option<bool>,
}

impl From<Claims> for ValidClaims {
  fn from(claims: Claims) -> Self {
    ValidClaims {
      sub: claims.sub,
      iat: claims.iat,
      exp: claims.exp,
      phone: claims.phone,
    }
  }
}

pub fn validate_claims(
  claims: &ValidClaims,
  user_item: &UserTable,
) -> std::result::Result<(), ValidationError> {
  let user_id = claims.sub.as_str();
  let user_phone = claims.phone.as_str();
  let exp = claims.exp;
  let iat = claims.iat;

  let error = ValidationError::new("validate_claims");

  match user_id == user_item.user_id.to_string() {
    // 用户id正确
    true => {
      match user_phone == user_item.user_phone {
        // 手机号正确
        true => {
          match exp > iat {
            // 过期时间正确
            true => Ok(()),
            // 过期时间错误
            false => Err(error.with_message("该用户登录状态失效.".into())),
          }
        }
        // 手机号错误
        false => Err(ValidationError::new("用户ID与手机号不匹配.".into())),
      }
    }
    // 用户ID错误
    false => Err(error.with_message("该用户未注册".into())),
  }
}
