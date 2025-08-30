use std::sync::{Arc, Mutex};
use axum::{middleware, Extension, Json, Router};
use axum::extract::{Path, State};
use axum::routing::{get, patch, post};
use tracing::instrument;
use crate::middleware::auth_check_middleware;
use crate::model::app_state::AppState;
use crate::model::auth::Profile;
use crate::model::pg::delivery_person::{ClientProfile, UpdateStatusRequestBody};
use crate::model::task::{RequestBodyForTaskCreate, Task};
use crate::service::{profile_update, task_create};

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
      .route("/profile", patch(update_profile_handler))
}

#[instrument]
#[axum::debug_handler]
async fn update_profile_handler(
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(app_state): State<AppState>,
  Json(profile_body): Json<UpdateStatusRequestBody>,
) -> crate::api::error::Result<Json<ClientProfile>> {
  Ok(profile_update::service(profile, app_state, profile_body).await?)
}

