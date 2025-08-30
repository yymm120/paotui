use crate::model::pg::error::QueryError;
use crate::utils::error::ToolError;
use axum::http;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::debug;
use uuid::Uuid;

// Result
pub type Result<T> = core::result::Result<T, ServiceError>;

#[derive(Debug, strum_macros::AsRefStr, Error)]
pub enum ServiceError {
  #[error(transparent)]
  Error(#[from] anyhow::Error),
  #[error(transparent)]
  HttpError(#[from] http::Error),
  #[error(transparent)]
  DataBaseError(#[from] sqlx::Error),

  #[error(transparent)]
  ToolError(#[from] ToolError),
  // UuidError(Uuid::),
  #[error("认证失败! {0}")]
  UnauthorizedError(&'static str),

  #[error("Deduction failed!")]
  PaymentDeductionFailed(&'static str),

  #[error("DB db failed! {0}")]
  DBOperationFailed(String),

  #[error(transparent)]
  QueryError(#[from] QueryError),
}
// impl From<Uuid::Error> for ServiceError {
//   fn from(val: Uuid::Error) -> Self {
//     ServiceError::HttpError(val)
//   }
// }

impl IntoResponse for ServiceError {
  fn into_response(self) -> Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    // Create a placeholder Axum response.
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the response.
    // response.extensions_mut().insert(self);

    response
  }
}

// impl std::error::Error for ServiceError {}
