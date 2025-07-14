#![allow(unused)]
use crate::model::auth::UserType;
use crate::model::jwt::{Claims, DecodeData, EncodeData, ValidClaims};
use crate::model::table::user_table::UserTable;
use crate::utils::error::{Result, ToolError};
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Duration, Local};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use jsonwebtoken::{DecodingKey, Validation, decode};
use tracing::{debug, info, instrument};
use uuid::Uuid;
use validator::{Validate, ValidateArgs};

pub fn generate_token(user_type: UserType, claims: Claims) -> Result<String> {
  match user_type {
    UserType::VerifiedUser => generate_token_for_verified_user(claims),
    UserType::UnsignedUpUser => generate_token_for_unsigned_up_user(claims),
    UserType::UnSignedInUser => generate_token_for_unsigned_in_user(claims),
  }
}

#[instrument]
pub fn generate_claims(user_type: UserType, user_item: Option<UserTable>) -> Claims {

  match user_type {
    UserType::VerifiedUser => {
      let user = user_item.expect("user must exist. user type is verified user.");
      generate_claims_for_verified_user(user)
    },
    UserType::UnsignedUpUser => {
      debug!("user type is unsigned up.");
      generate_claims_for_unsigned_up_user()
    },
    UserType::UnSignedInUser => {
      let user = user_item.expect("user must exist. user type is unsigned in user.");
      generate_claims_for_unsigned_in_user(user)
    },
  }
}

fn generate_claims_for_exist_user(user: UserTable) -> Claims {
  generate_claims_for_unsigned_in_user(user)
}

fn generate_claims_for_unsigned_up_user() -> Claims {
  let invalid_exp = generate_invalid_exp();
  Claims {
    sub: Uuid::new_v4().to_string(),
    exp: invalid_exp.exp,
    iat: invalid_exp.iat,
    phone: "".to_string(),
  }
}

fn generate_claims_for_unsigned_in_user(user: UserTable) -> Claims {
  let invalid_exp = generate_invalid_exp();
  Claims {
    sub: user.user_id.to_string(),
    exp: invalid_exp.exp,
    iat: invalid_exp.iat,
    phone: user.user_phone.to_string(),
  }
}

fn generate_claims_for_verified_user(user: UserTable) -> Claims {
  let valid_exp = generate_valid_exp();
  let claims = Claims {
    sub: user.user_id.to_string(),
    exp: valid_exp.exp,
    iat: valid_exp.iat,
    phone: user.user_phone.to_string(),
  };
  claims
}

async fn generate_token_for_exist_user(claims: Claims) -> Result<String> {
  generate_token_for_unsigned_in_user(claims)
}
/// TODO
fn generate_token_for_unsigned_up_user(claims: Claims) -> Result<String> {
  let key = b"your_secret_key";
  let now = Local::now();
  let exp = now.timestamp() - 1;
  let iat = now.timestamp();
  let unsigned_up_claims = Claims {
    sub: claims.sub,
    exp,
    iat,
    phone: claims.phone,
  };
  Ok(encode(
    &Header::default(),
    &unsigned_up_claims,
    &EncodingKey::from_secret(key),
  )?)
}
/// TODO
fn generate_token_for_unsigned_in_user(claims: Claims) -> Result<String> {
  generate_token_for_unsigned_up_user(claims)
}

/// TODO
fn generate_token_for_verified_user(claims: Claims) -> Result<String> {
  let key = b"your_secret_key";
  let now = Local::now();
  let expires_in = now + Duration::days(180);
  let exp = expires_in.timestamp();
  let iat = now.timestamp();
  let verified_claims = Claims {
    sub: claims.sub,
    exp,
    iat,
    phone: claims.phone,
  };
  Ok(encode(
    &Header::default(),
    &verified_claims,
    &EncodingKey::from_secret(key),
  )?)
}

// pub fn generate_claims_with_user_id()

/// 从exp中获取剩余天数
pub fn remaining_days(exp: i64) -> i64 {
  let now = Local::now().timestamp();
  let remaining_seconds = exp - now;

  // 转换为天数（向上取整）
  if remaining_seconds <= 0 {
    0 // 已过期
  } else {
    (remaining_seconds as f64 / 86400.0).ceil() as i64 // 86400秒=1天
  }
}

pub async fn encode_handler(Json(encode_data): Json<EncodeData>) -> impl IntoResponse {
  let key = b"your_secret_key";
}

pub async fn decode_handler(
  Json(decode_data): Json<DecodeData>,
) -> crate::utils::error::Result<Json<Claims>> {
  let key = b"your_secret_key";

  debug!("{}", serde_json::to_string_pretty(&decode_data).unwrap());
  // 基本jwt检查
  // 在 jsonwebtoken 库中，Validation 结构体用于配置 JWT 的验证规则
  let mut validation = Validation::new(Algorithm::HS256);
  // // 检查过期时间
  // validation.validate_exp = true;
  validation.validate_aud = false;
  // 不检查过期时间, 通过claims.exp的值手动检查
  validation.validate_exp = false;
  validation.validate_nbf = false;

  let token = decode::<Claims>(
    &(decode_data.token.strip_prefix("Bearer ").unwrap()),
    &DecodingKey::from_secret(key),
    &validation,
  )
  .map(|data| Some(data.claims));

  let claims = token?.unwrap();

  Ok(Json(claims))
}

pub fn update_exp_and_iat(mut claims: Claims) -> Claims {
  // 获取当前本地时间
  let now = Local::now();
  // 计算 180 天后的时间
  let expires_in = now + Duration::days(180);
  // 转换为 Unix 时间戳
  let exp = expires_in.timestamp();
  let now = now.timestamp();

  claims.exp = exp;
  claims.iat = now;

  claims
}

/// 验证jwt
pub fn verify_jwt(token: Option<&str>) -> Result<Option<Claims>> {
  let key = b"your_secret_key";

  // 基本jwt检查
  // 在 jsonwebtoken 库中，Validation 结构体用于配置 JWT 的验证规则
  let mut validation = Validation::new(Algorithm::HS256);
  // // 检查过期时间
  validation.validate_aud = false;
  // 不检查过期时间, 通过claims.exp的值手动检查
  validation.validate_exp = false;
  validation.validate_nbf = false;

  let claims = decode::<Claims>(
    token.unwrap_or(""),
    &DecodingKey::from_secret(key),
    &validation,
  )
  .map(|data| data.claims)?;

  // let valid_claims = ValidClaims::from(claims.clone());
  // valid_claims.validate_with_args()?;
  Ok(Some(claims))
}

struct IatExp {
  iat: i64,
  exp: i64,
}

fn generate_valid_exp() -> IatExp {
  let now = Local::now();
  // 计算 180 天后的时间
  let expires_in = now + Duration::days(180);
  let exp = expires_in.timestamp();
  let iat = now.timestamp();
  IatExp { iat, exp }
}

fn generate_invalid_exp() -> IatExp {
  let now = Local::now();
  let iat = now.timestamp();
  let exp = iat - 1;
  IatExp { iat, exp }
}

// 单元测试
#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{Duration, Local};

  #[test]
  fn test_remaining_days() {
    let future = (Local::now() + Duration::days(5)).timestamp();
    assert_eq!(remaining_days(future), 5);

    let past = (Local::now() - Duration::days(1)).timestamp();
    assert_eq!(remaining_days(past), 0);
  }

  #[test]
  fn test_generate_claims() {
    //
  }
}

