use crate::AppState;
use crate::model::auth::{Profile, RequestBodyForLogin, ResponseForLogin, UserType};
use crate::model::pg::user::{UserForCreate, User};
use crate::model::pg::user::{insert};
use crate::service::{
  common::verify_login_code,
  error::{Result, ServiceError},
};
use crate::utils::db::Db;
use crate::utils::jwt::{generate_claims, generate_token};
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Response;
use axum::{Extension, Json, http};
use sqlx::Error;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use fake::Fake;
use fake::faker::name::zh_cn::Name;
use fake::locales::ZH_CN;
use bigdecimal::Decimal;
use tracing::{debug, instrument};
use uuid::Uuid;
use crate::model::app_state::AppIdentify;
use crate::model::jwt::ClaimsUser;
use crate::model::pg::customer_person_table::CustomerPersonTable;
use crate::model::pg::delivery_app_user::CreateDeliveryAppUser;
use crate::model::pg::delivery_person::CreateDeliveryPerson;
use crate::model::pg::delivery_app_user::{create_delivery_app_user, query_delivery_app_user_by_phone};
use crate::model::pg::delivery_person::create_delivery_person;
use crate::utils::time::now_time;

// #[instrument]
pub async fn login(
  header_map: HeaderMap<HeaderValue>,
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  login_body: RequestBodyForLogin,
) -> Result<Json<ResponseForLogin>> {
  let code = login_body.code;
  let db = app_state.db;
  let profile_clone;
  let token;
  let user_type;
  {
    let mut profile_guard = profile.lock().unwrap();
    profile_clone = profile_guard.clone();
    token = profile_guard.token.clone();
    user_type = profile_guard.user_type.clone();
  }

  // 检查用户类型
  match profile_clone.user_type {
    // 无需继续登录, 直接返回 TODO: 已登录用户也许不应该获取登录验证码, 在code_handler中就应该阻止
    UserType::VerifiedUser => Ok(Json::from(ResponseForLogin { status: false })),
    // 检查短信验证码, 创建用户
    UserType::UnsignedUpUser | UserType::UnSignedInUser => {
      let user_phone = login_body.user_phone;
      let exist_user = query_delivery_app_user_by_phone(&db, user_phone.clone()).await;
      match exist_user {
        // 果然存在, 这是未登录用户, 现在需要登录
        Ok(user_item) => {
          let status = verify_login_code(user_phone.as_str(), code.as_str())
            .await
            .unwrap_or(false);
          match status {
            true => {

              let claims = generate_claims(UserType::VerifiedUser, Some(ClaimsUser::from(user_item.clone())));
              let token = generate_token(UserType::VerifiedUser, claims.clone())?;

              {
                let mut profile = profile.lock().unwrap();
                profile.user_id = user_item.user_id.to_string();
                profile.user_phone = user_phone;
                profile.claims = claims;
                profile.token = token;
                profile.user_type = UserType::VerifiedUser;
              }

              Ok(Json::from(ResponseForLogin { status: true }))
            }
            false => Err(ServiceError::UnauthorizedError("验证码错误!")),
          }
        }
        Err(e) => {
          let status = verify_login_code(user_phone.as_str(), code.as_str())
            .await
            .unwrap_or(false);
          match status {
            true => {
              let new_user_item = create_delivery_app_user(&db, CreateDeliveryAppUser {
                username: format!("Random-{:?}", Name().fake::<String>()),  // 随机
                password: "123456".to_string(), // 默认
                telephone: user_phone,
                money: Default::default(),
                status: "resting".to_string(),
              }).await?;

              let claims = generate_claims(UserType::VerifiedUser, Some(ClaimsUser::from(new_user_item.clone())));
              let token = generate_token(UserType::VerifiedUser, claims.clone())?;
              {
                let mut profile = profile.lock().unwrap();
                profile.user_phone = new_user_item.telephone;
                profile.user_id = new_user_item.user_id.to_string();
                profile.user_type = UserType::VerifiedUser;
                profile.claims = claims.clone();
                profile.token = token;
              }
              Ok(Json::from(ResponseForLogin { status: true }))
            }
            false => Err(ServiceError::UnauthorizedError("验证码错误!")),
          }
        }
      }
    }
  }
}

async fn login_unsigned_in_user(
  user_item: User,
  token: String,
  db: &Db,
  code: &str,
) -> Result<Json<ResponseForLogin>> {
  // 校验登录验证码
  let status = verify_login_code(user_item.telephone.as_str(), code).await;
  match status.unwrap_or(false) {
    // 验证码正确
    true => {
      let claims = generate_claims(UserType::VerifiedUser, Some(ClaimsUser::from(user_item)));
      let token = generate_token(UserType::VerifiedUser, claims)?;
      Ok(Json::from(ResponseForLogin { status: false }))
    }
    // 验证码错误
    false => {
      let claims = generate_claims(UserType::UnSignedInUser, Some(ClaimsUser::from(user_item)));
      let token = generate_token(UserType::UnSignedInUser, claims)?;
      Ok(Json::from(ResponseForLogin { status: false }))
    }
  }
}

// async fn login_verified_user(token: String) -> Result<Json<ResponseForLogin>> {
//   Ok(Json::from(ResponseForLogin { status: false }))
//   // Ok(Response::builder()
//   //   .status(StatusCode::OK)
//   //   .header(AUTHORIZATION, token)
//   //   .header("Content-Type", "application/json")
//   //   .body::<ResponseForLogin>(ResponseForLogin { status: true })?)
// }

async fn login_unsigned_up_user(
  user_item: User,
  token: String,
  db: &Db,
  code: &str,
) -> Result<Json<ResponseForLogin>> {
  // let user_id = profile.user_id.clone();
  // let user_phone = profile.user_phone.clone();
  //
  // let claims_a = match user_item_res {
  //   Ok(user_item) => {
  //     generate_claims(UserType::VerifiedUser)
  //
  //   }
  //   Err(e) => {}
  // };
  //
  // let claims_op = generate_claims(user_item);
  // let token = generate_token(profile.user_type.clone(), &claims_op);
  //
  // let token = profile.clone().token;
  Ok(Json::from(ResponseForLogin { status: false }))
}
