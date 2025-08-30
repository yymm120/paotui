#![allow(unused)]
use crate::AppState;
use crate::api::error::{ApiError, Result};
use crate::model::auth::{
  Profile, RequestBodyForCode, RequestBodyForLogin, ResponseForCode, ResponseForInitial,
  ResponseForLogin, UserInfo, UserType,
};
// use crate::model::jwt::{Claims, ValidClaims};
use crate::model::jwt::Claims;
use crate::service::auth_init_service::init;
use crate::service::auth_login_service;
use crate::service::common::{create_profile_by_token, get_token_from_header};
use crate::utils::db::{Db, new_db_pool};
use crate::utils::jwt::{
  generate_claims, generate_token, remaining_days, update_exp_and_iat, verify_jwt,
};
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router, middleware};
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Error, Postgres};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::registry::ExtensionsMut;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};
// #[instrument]
#[axum::debug_middleware]
pub async fn auth_check(
  header_map: HeaderMap<HeaderValue>,
  State(app_state): State<AppState>,
  mut request: Request,
  next: Next,
) -> Result<impl IntoResponse> {
  let db = app_state.db;
  let app_identify = app_state.app_identify;
  let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
  let profile = create_profile_by_token(header_token.clone(), &db, app_identify).await;
  let profile_arc = Arc::new(Mutex::new(profile.clone()));


  request
    .extensions_mut()
    .insert::<Arc<Mutex<Profile>>>(profile_arc.clone());


  let response = next_run(request, next, &profile).await?;
  let mut response = response.into_response();

  let profile_guard = profile_arc.lock().unwrap();
  let user_type = profile_guard.user_type.clone();

  let profile_token = profile_guard.token.clone();
  // 安全设置新的Authorization头
  if profile_token != header_token.clone() {
    let verified_token = format!("Bearer {}", &profile_token);
    response
      .headers_mut()
      .insert(AUTHORIZATION, HeaderValue::from_str(&verified_token).unwrap());
  }

  // debug!("After Layer");
  Ok(response)
}


pub async fn next_run(mut request: Request, next: Next, profile: &Profile) -> Result<Response> {
  let path = request.uri().path();
  if path == "/api/_" || path == "/api/login" || path == "/api/code" {
    Ok(next.run(request).await)
  } else {
    match profile.user_type {
      UserType::VerifiedUser => {
        Ok(next.run(request).await)
      }
      _ => {
        Err(ApiError::Unauthorized(profile.user_id.to_string()))
      }
    }
  }
}