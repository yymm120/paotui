use std::sync::{Arc, Mutex};
use axum::{middleware, Extension, Json, Router};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::routing::{delete, get, patch, post, put};
use tracing::instrument;
use crate::middleware::auth_check_middleware;
use crate::middleware::auth_check_middleware::auth_check;
use crate::model::app_state::AppState;
use crate::model::auth::{Profile, RequestBodyForCode, RequestBodyForLogin, ResponseForLogin};
use crate::model::task::{RequestBodyForTaskCreate, RequestBodyForTaskUpdate};
use crate::service::{task_cancel, task_complete, task_create, task_query, task_update};

pub fn routes(app_state: AppState) -> Router<AppState> {
  Router::new().nest(
    "/api",
    Router::new()
      .route("/task", post(create_task_handler))
      .route("/task", get(query_all_task_handler))
      .route("/task/:id", get(query_task_handler))
      .route("/task/:id", delete(cancel_task_handler))
      .route("/task/:id", put(completed_task_handler))
      .route("/task/:id", patch(update_task_handler))
      .layer(middleware::from_fn_with_state(
        app_state,
        auth_check,
      )),
  )
}

#[instrument]
#[axum::debug_handler]
async fn create_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Json(task_body): Json<RequestBodyForTaskCreate>
) {
  task_create::mock_service(profile, app_state, task_body).await
}

#[instrument]
#[axum::debug_handler]
async fn query_all_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
) {
  task_query::mock_service(profile, None).await
}
#[instrument]
#[axum::debug_handler]
async fn query_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32> 
) {
  task_query::mock_service(profile, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn cancel_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32>
) {
  task_cancel::mock_service(profile, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn completed_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32>
) {
  task_complete::mock_service(profile, Some(id)).await
}

#[instrument]
#[axum::debug_handler]
async fn update_task_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  Path(id): Path<i32>,
  Json(task_body): Json<RequestBodyForTaskUpdate>
) {
  task_update::mock_service(profile, Some(id), task_body).await
}