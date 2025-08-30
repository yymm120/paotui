use axum::{middleware, Router};
use axum::routing::{get, post};
use crate::middleware::auth_check_middleware;
use crate::model::app_state::AppState;

pub mod auth;
pub mod error;
pub mod task;
pub mod validate;
pub mod profile;




pub fn routes(app_state: AppState) -> Router<AppState> {
  let api_routes = Router::new()
    .merge(auth::routes(app_state.clone()))
    .merge(task::routes(app_state.clone()))
    .merge(profile::routes(app_state.clone()));
  Router::new().nest(
    "/api",
    api_routes
  )
    .layer(
      middleware::from_fn_with_state(
        app_state, auth_check_middleware::auth_check
      ))
}