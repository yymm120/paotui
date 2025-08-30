use crate::model::auth::{Profile, ResponseForInitial, UserInfo, UserType};
use crate::service::error::Result;
use crate::utils::db::Db;
use axum::Json;
use axum::http::HeaderMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument};

#[instrument]
pub async fn init(
  x: &Db,
  header_map: HeaderMap,
  profile: Arc<Mutex<Profile>>,
) -> Result<Json<ResponseForInitial>> {
  let user_type;
  let claims_op;
  {
    let profile_guard = profile.lock().unwrap();
    user_type = profile_guard.user_type.clone();
    claims_op = Some(profile_guard.claims.clone());
  }

  let claims = claims_op.as_ref().unwrap();

  let res = match user_type {
    // 已登录用户
    UserType::VerifiedUser => Json(ResponseForInitial::login_success_response(UserInfo {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
    // 未注册用户
    UserType::UnsignedUpUser => Json(ResponseForInitial::login_failed_response(UserInfo {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
    // 未登录用户
    UserType::UnSignedInUser => Json(ResponseForInitial::login_failed_response(UserInfo {
      user_id: claims.sub.to_string(),
      user_type,
      phone_number: claims.phone.to_string(),
    })),
  };

  Ok(res)
}
