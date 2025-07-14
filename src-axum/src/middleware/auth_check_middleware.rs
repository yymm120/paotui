#![allow(unused)]
use crate::AppState;
use crate::api::error::{ApiError, Result};
use crate::model::auth::{
  Profile, RequestBodyForCode, RequestBodyForLogin, ResponseForCode, ResponseForInitial,
  ResponseForLogin, User, UserType,
};
// use crate::model::jwt::{Claims, ValidClaims};
use crate::model::jwt::Claims;
use crate::model::table::user_table::UserTable;
use crate::service::auth_login_service;
use crate::service::common::{create_profile_by_auth_token, get_token_from_header};
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
use crate::service::auth_init_service::init;
#[instrument]
#[axum::debug_middleware]
pub async fn auth_check(
  header_map: HeaderMap<HeaderValue>,
  State(app_state): State<AppState>,
  mut request: Request,
  next: Next,
) -> Result<impl IntoResponse> {

  let db = app_state.db;
  let header_token = get_token_from_header(&header_map).unwrap_or("").to_string();
  let profile = create_profile_by_auth_token(header_token.clone(), &db).await;
  let profile_token = profile.token.clone();

  request.extensions_mut().insert(Arc::new(Mutex::new(profile)));

  let response = next.run(request).await;
  let mut response = response.into_response();

  // 安全设置新的Authorization头
  if profile_token != header_token.clone() {
    if let Ok(header_value) = HeaderValue::from_str(&format!("Bearer {}", &profile_token)) {
      response
        .headers_mut()
        .insert(header::AUTHORIZATION, header_value);
    }
  }

  debug!("After Layer");
  Ok(response)
}