use std::sync::{Arc, Mutex};
use axum::http::HeaderMap;
use axum::Json;
use tracing::{debug, instrument};
use crate::model::auth::{Profile, ResponseForInitial, User, UserType};
use crate::utils::db::Db;
use crate::service::error::Result;

#[instrument]
pub async fn init(x: &Db, header_map: HeaderMap, profile: Arc<Mutex<Profile>>) -> Result<Json<ResponseForInitial>>{

  // 检查header中token, 如果正确, 就说明是登录用户, 否则是未登录用户
  let user_type;
  let claims_op;
  {
    let profile_guard = profile.lock().unwrap();
    user_type = profile_guard.user_type.clone();
    claims_op = Some(profile_guard.claims.clone());
    debug!("claims: {:?}", claims_op.clone().unwrap())
  }

  let claims = claims_op.as_ref().unwrap();

  let res = match user_type {
    // 已登录用户
    UserType::VerifiedUser => Json(ResponseForInitial::login_success_response(User {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
    // 未注册用户
    UserType::UnsignedUpUser => Json(ResponseForInitial::login_failed_response(User {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
    // 未登录用户
    UserType::UnSignedInUser => Json(ResponseForInitial::login_failed_response(User {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
  };

  debug!("end init_handler");
  Ok(res)
}