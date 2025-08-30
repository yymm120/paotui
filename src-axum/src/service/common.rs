use crate::model::auth::{Profile, UserInfoForAutoLogin, UserType};
use crate::model::jwt::{Claims, ClaimsUser, ValidClaims, ValidateContext};
use crate::model::pg::user::User;
use crate::model::pg::user::{query_user, query_user_by_id, query_user_by_phone};
use crate::utils::db::Db;
use crate::utils::jwt::{generate_claims, generate_token, remaining_days, verify_jwt};
use axum::http::{HeaderMap, HeaderValue, header};
use chrono::Local;
use std::str::FromStr;
use tracing::{debug, error};
use validator::{ValidateArgs, ValidationErrors};
use crate::model::app_state::AppIdentify;
use crate::model::pg::delivery_app_user::{query_delivery_app_user_by_phone, query_delivery_app_user_by_user_id};

pub async fn verify_login_code(user_phone: &str, code: &str) -> anyhow::Result<bool> {
  if code == "1234" {
    Ok(true)
  } else {
    Err(anyhow::Error::msg("wrong auth token"))
  }
}
pub async fn create_profile_by_token(header_token: String, db: &Db, app_identify: AppIdentify) -> Profile {
  match app_identify {
    AppIdentify::DeliveryApp => {
      create_delivery_app_profile_by_token(header_token, db).await
    }
    AppIdentify::TaskApp => {
      create_unknown_app_profile().await
    }
  }
}

pub async fn create_unknown_app_profile() -> Profile {
  Profile::create_profile(UserType::UnsignedUpUser, None)
}

/// ## check_auth_token
/// 检查Header中的token，并返回相应的user_type和claims
///
/// 1. 如果用户没有携带token, 或者用户token格式错误, 那么重新生成token, user_id会在这里生成.
/// 2. 格式正确, 如果user_id存在, 则检查exp过期时间.
/// 3. 格式正确, 如果user_id不存在, 则更新过期时间为 `now - 1s`./!
///
///
/// | user_type        | user_id  | user_phone | iat   | exp      |
/// | ---------------- | -------- | ---------- | ----- | -------- |
/// | `UnsignedUpUser` | generate | empty      | now   | now - 1s |
/// | `UnsignedInUser` | exist    | exist      | now   | now - 1s |
/// | `VerifiedUser`   | exist    | exist      | exist | exist    |
///
pub async fn create_delivery_app_profile_by_token(header_token: String, db: &Db) -> Profile {
  let mut user_type = UserType::UnsignedUpUser;
  let mut claims_res = Claims::default();

  let mut token = header_token;

  // web token json 的格式校验
  let verified_token = verify_jwt(Some(token.as_ref()));

  match verified_token {
    Ok(claims_option) => {
      match claims_option {
        Some(claims) => {

          let user_id = i64::from_str(&claims.sub).unwrap_or(0);
          if (user_id == 0) {
            return Profile::create_profile(UserType::UnsignedUpUser, None);
          }

          let user_res = query_delivery_app_user_by_user_id(db, user_id).await;

          match user_res {
            Err(e) => Profile::create_profile(UserType::UnsignedUpUser, None),
            Ok(delivery_app_user) => {
              let valid_claims = ValidClaims::from(claims.clone());
              match valid_claims.validate_with_args(&ValidateContext::from(delivery_app_user.clone())) {
                Ok(_) => {Profile::create_profile(UserType::VerifiedUser, Some(ClaimsUser::from(delivery_app_user)))}
                Err(e) => {
                  debug!("validation_error: {:?}", e);
                  Profile::create_profile(UserType::UnSignedInUser, Some(ClaimsUser::from(delivery_app_user)))
                }
              }
            }
          }
        }
        None => Profile::create_profile(UserType::UnsignedUpUser, None),
      }
    }
    Err(e) => Profile::create_profile(UserType::UnsignedUpUser, None),
  }
}

pub fn get_token_from_header(header_map: &HeaderMap<HeaderValue>) -> Option<&str> {
  let token_option = header_map
    .get(header::AUTHORIZATION)
    .and_then(|h| h.to_str().ok())
    .and_then(|s| s.strip_prefix("Bearer "));

  token_option
}
//
// #[cfg(test)]
// mod tests {
//   use crate::model::auth::UserType;
//   use crate::model::table::user::User;
//   use crate::service::common::{create_delivery_app_profile_by_token, get_token_from_header};
//   use crate::utils::db::new_db_pool;
//   use crate::utils::jwt::{generate_claims, generate_token};
//   use axum::http::header::AUTHORIZATION;
//   use axum::http::{HeaderMap, HeaderValue};
//   use sqlx::postgres::PgPoolOptions;
//   use std::time::Duration;
//   use tracing_subscriber::fmt::FormatFields;
//   use crate::model::jwt::ClaimsUser;
//   use crate::model::pg::user::User;
//
//   #[tokio::test]
//   async fn test_fn_check_auth_token() -> anyhow::Result<()> {
//     let db = new_db_pool("postgres://postgres:postgres@localhost/paotui").await?;
//     // 1. 测试不携带token执行 check_auth_token() 函数
//     let mut header_map = HeaderMap::default();
//     header_map.append(AUTHORIZATION, HeaderValue::from_str("")?);
//     let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
//     let profile = create_delivery_app_profile_by_token(header_token.clone(), &db).await;
//     // println!("user_type: {}, claims_op: {:?}", user_type, claims_op);
//     assert_eq!(
//       profile.user_type.to_string(),
//       UserType::UnsignedUpUser.to_string()
//     );
//     assert_eq!(
//       profile.claims.clone().exp.clone() + 1,
//       profile.claims.clone().iat
//     );
//
//     // 2. 测试携带UserType::UnsignedUpUser时, 执行 check_auth_token() 函数
//     let token = generate_token(UserType::UnsignedUpUser, profile.claims.clone())?;
//     // println!("token: {}", &token);
//     header_map.insert(
//       AUTHORIZATION,
//       HeaderValue::from_str(format!("Bearer {}", token.as_str()).as_str())?,
//     );
//     let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
//     let profile = create_delivery_app_profile_by_token(header_token, &db).await;
//     // println!("user_type: {}, claims_op: {:?}", user_type, claims_op);
//     assert_eq!(
//       profile.user_type.to_string(),
//       UserType::UnsignedUpUser.to_string()
//     );
//     assert_eq!(
//       profile.claims.clone().exp.clone() + 1,
//       profile.claims.clone().iat
//     );
//
//     // 3. 测试携带UserType::VerifiedUser时, 执行 check_auth_token() 函数
//     let user = sqlx::query_as!(
//       User,
//       "SELECT * FROM public.user WHERE telephone = $1",
//       "8617815349593"
//     )
//     .fetch_one(&db)
//     .await?;
//     // println!("user: {:?}", user);
//     let claims = generate_claims(UserType::VerifiedUser, Some(ClaimsUser::from(user.clone())));
//     let token = generate_token(UserType::VerifiedUser, claims)?;
//     header_map.insert(
//       AUTHORIZATION,
//       HeaderValue::from_str(format!("Bearer {}", token.as_str()).as_str())?,
//     );
//     let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
//
//     let profile = create_delivery_app_profile_by_token(header_token.clone(), &db).await;
//     // println!("user_type: {}, claims_op: {:?}", user_type, claims_op);
//
//     // 3. 测试携带UserType::UnsignedInUser时, 执行 check_auth_token() 函数
//     let claims = generate_claims(UserType::UnSignedInUser, Some(ClaimsUser::from(user.clone())));
//     let token = generate_token(UserType::UnSignedInUser, claims)?;
//     header_map.insert(
//       AUTHORIZATION,
//       HeaderValue::from_str(format!("Bearer {}", token.as_str()).as_str())?,
//     );
//
//     let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
//     let profile = create_delivery_app_profile_by_token(header_token, &db).await;
//     // println!("user_type: {}, claims_op: {:?}", user_type, claims_op);
//     Ok(())
//   }
// }
