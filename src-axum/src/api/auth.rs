#![allow(unused)]

use crate::AppState;
use crate::api::error::{ApiError, Result};
use crate::middleware::auth_check_middleware;
use crate::model::auth::{
  Profile, RequestBodyForCode, RequestBodyForLogin, ResponseForCode, ResponseForInitial,
  ResponseForLogin, User, UserType,
};
use crate::model::jwt::Claims;
use crate::model::table::user_table::UserTable;
use crate::service::auth_login_service;
use crate::service::common::{create_profile_by_auth_token, get_token_from_header};
use crate::utils::db::{Db, new_db_pool};
use crate::utils::jwt::{
  generate_claims, generate_token, remaining_days, update_exp_and_iat, verify_jwt,
};
use axum::body::Body;
use axum::extract::{ConnectInfo, Request, State};
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
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
// use tower_http::trace::TraceLayer;
use crate::service::auth_init_service::init;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::registry::ExtensionsMut;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

pub fn routes(app_state: AppState) -> Router<AppState> {
  Router::new().nest(
    "/api",
    Router::new()
      // .with_state(db.clone())
      // 启动应用时检查是否有jwt
      .route("/_", get(init_handler))
      // 获取验证码
      .route("/code", post(code_handler))
      // 用验证码登录
      .route("/login", post(login_handler))
      .layer(middleware::from_fn_with_state(
        app_state,
        auth_check_middleware::auth_check,
      )),
  )
}

#[instrument]
#[axum::debug_handler]
async fn login_handler(
  header_map: HeaderMap<HeaderValue>,
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Json(login_body): Json<RequestBodyForLogin>,
) -> Result<Json<ResponseForLogin>> {
  debug!("into login_handler!");
  let res = auth_login_service::login(header_map, profile, app_state, login_body).await?;
  debug!("into login_handler!");
  // res
  Ok(res)
}

#[instrument]
#[axum::debug_handler]
async fn code_handler(Json(code_body): Json<RequestBodyForCode>) -> Result<Json<ResponseForCode>> {
  // 向手机发送验证码
  let phone_number = code_body.phone_number;
  // 返回状态
  Ok(Json(ResponseForCode {
    code: "1234".to_string(),
  }))
}

#[instrument]
#[axum::debug_handler]
async fn init_handler(
  State(app_state): State<AppState>,
  header_map: HeaderMap<HeaderValue>,
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  // Json(user): Json<UserInfoForAutoLogin>,
) -> Result<Json<ResponseForInitial>> {
  debug!("start init_handler!");
  let db = app_state.db;
  let res = init(&db, header_map, profile).await?;
  debug!("end init_handler!");
  Ok(res)
}

#[cfg(test)]
mod tests {
  use crate::model::app_state::AppState;
  use crate::utils::db::new_db_pool;
  use crate::{app_routes, application};
  use axum::body::Body;
  use axum::http;
  use axum::http::header::AUTHORIZATION;
  use axum::http::{HeaderMap, Method, Request, StatusCode};
  use http_body_util::BodyExt;
  use serde_json::json;
  // for `collect`
  use tower::{Service, ServiceExt};
  use crate::model::auth::UserType;
  use crate::model::table::user_table::UserTable;
  use crate::service::common::get_token_from_header;
  use crate::utils::jwt::{generate_claims, generate_token, verify_jwt};
  // for `call`, `oneshot`, and `ready`

  #[tokio::test]
  async fn test_auth() -> anyhow::Result<()> {
    let db = new_db_pool("postgres://postgres:postgres@localhost/paotui").await?;
    let app_state = AppState { db };

    let app = app_routes(app_state).await;

    let request = Request::builder()
      .uri("/api/_")
      .header(AUTHORIZATION, "")
      .method(Method::GET)
      .body(Body::empty())?;

    let response = app.clone().oneshot(request).await.unwrap();

    let token = response.headers().get(AUTHORIZATION).unwrap().to_owned();
    println!("token: {:?}", token);

    let db = new_db_pool("postgres://postgres:postgres@localhost/paotui").await?;
    let user = sqlx::query_as!(
      UserTable,
      "SELECT * FROM user_table WHERE user_phone = $1",
      "8617815349593"
    )
      .fetch_one(&db)
      .await?;

    let claims = generate_claims(UserType::VerifiedUser, Some(user));
    let token = generate_token(UserType::VerifiedUser, claims)?;
    println!("authed token:: {}", token);
    let request = Request::builder()
      .uri("/api/_")
      .header(AUTHORIZATION, format!("Bearer {}", token))
      .method(Method::GET)
      .body(Body::empty())?;

    let response = app.clone().oneshot(request).await?;
    let token = response.headers().get(AUTHORIZATION).unwrap().to_owned();
    println!("authed token: {:?}", token);

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("body: {:?}", body);
    Ok(())
  }

  #[tokio::test]
  async fn test_handler_code_handler() -> anyhow::Result<()> {
    let db = new_db_pool("postgres://postgres:postgres@localhost/paotui").await?;
    let app_state = AppState { db };

    let app = app_routes(app_state).await;

    let request = Request::builder()
      .uri("/api/code")
      .header(AUTHORIZATION, "")
      .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
      .method(Method::POST)
      .body(Body::from(serde_json::to_vec(&json!({
        "phone_number": "1".to_string(),
      }))?))?;

    let response = app.clone().oneshot(request).await.unwrap();

    let token = response.headers().get(AUTHORIZATION).unwrap().to_owned();
    println!("token: {:?}", token);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("body: {:?}", body);
    Ok(())
  }

  #[tokio::test]
  async fn test_handler_login_handler() -> anyhow::Result<()> {
    let db = new_db_pool("postgres://postgres:postgres@localhost/paotui").await?;
    let app_state = AppState { db };

    let app = app_routes(app_state).await;

    let request = Request::builder()
      .uri("/api/login")
      .header(AUTHORIZATION, "")
      .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
      .method(Method::POST)
      .body(Body::from(serde_json::to_vec(&json!({
        "user_phone": "861781534953".to_string(),
        "code": "1234".to_string(),
      }))?))?;

    let response = app.clone().oneshot(request).await.unwrap();

    let token = response.headers().get(AUTHORIZATION).unwrap().to_owned();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("body: {:?}", body);
    println!("token: {:?}", token.clone());

    let mut header_map = HeaderMap::new();
    header_map.insert(AUTHORIZATION, token);
    let token = get_token_from_header(&header_map).unwrap();
    println!("token: {:?}", token.clone());

    let request = Request::builder()
      .uri("/api/_")
      .header(AUTHORIZATION, format!("Bearer {}", token))
      .method(Method::GET)
      .body(Body::empty())?;

    let claims = verify_jwt(Some(token))?;
    let response = app.clone().oneshot(request).await.unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    println!("body: {:?}", body);
    Ok(())
  }
}

