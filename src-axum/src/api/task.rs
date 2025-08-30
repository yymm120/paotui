use crate::api::error::{ApiError, Result};
use crate::middleware::auth_check_middleware;
use crate::middleware::auth_check_middleware::auth_check;
use crate::model::app_state::AppState;
use crate::model::auth::{Profile, RequestBodyForCode, RequestBodyForLogin, ResponseForLogin};
use crate::model::task::{RequestBodyForTaskCreate, RequestBodyForTaskUpdate, Task};
use crate::service::{task_cancel, task_complete, task_create, task_query, task_update};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::routing::{delete, get, patch, post, put};
use axum::{Extension, Json, Router, middleware};
use std::sync::{Arc, Mutex};
use serde::Deserialize;
use tracing::{debug, instrument};

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
      .route("/task", post(create_task_handler))
      .route("/task", get(query_all_task_handler))
      .route("/task/{id}", get(query_task_handler))
      .route("/task/{id}", delete(cancel_task_handler))
      .route("/task/{id}", put(completed_task_handler))
      .route("/task/{id}", patch(update_task_handler))
}

#[instrument]
#[axum::debug_handler]
async fn create_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Json(task_body): Json<RequestBodyForTaskCreate>,
) -> Result<Json<Task>> {
  // task_create::mock_service(profile, app_state, task_body).await
  Ok(task_create::service(profile, app_state, task_body).await?)
}

/// query_all_task_handler
///
/// 查询所有未接取的任务, 以及属于自己的任务
#[instrument]
#[axum::debug_handler]
async fn query_all_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Query(params): Query<QueryParamsForListTask>,
) -> Result<Json<Vec<Task>>> {
  Ok(task_query::service(profile, None, params, app_state).await?)
}
#[instrument]
#[axum::debug_handler]
async fn query_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Path(id): Path<i32>,
) {
  task_query::mock_service(profile, app_state, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn cancel_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32>,
) {
  task_cancel::mock_service(profile, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn completed_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32>,
) {
  task_complete::mock_service(profile, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn update_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<String>,
  State(app_state): State<AppState>,
  Query(query): Query<QueryParamsForUpdateTask>,
) -> Result<Json<Task>> {
  Ok(task_update::service(profile, id, query.status.to_string(), app_state).await?)
}

#[derive(Debug, Deserialize)]
struct QueryParamsForUpdateTask {
  pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryParamsForListTask {
  pub status: Option<String>,
  pub owned: Option<bool>
}