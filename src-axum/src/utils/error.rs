#![allow(unused)]
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use tracing::debug;

// Result
pub type Result<T> = core::result::Result<T, ToolError>;

// Error
#[derive(Debug, Clone, strum_macros::AsRefStr, Error)]
pub enum ToolError {
  #[error("Error Message: {0}")]
  Error(&'static str),
  #[error(transparent)]
  TokenError(#[from] jsonwebtoken::errors::Error),
  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),
  // #[error("")]
  // (#[from] validator::ValidationErrors),
}

// impl core::fmt::Display for ToolError {
//   fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
//     write!(fmt, "{self:?}")
//   }
// }

impl IntoResponse for ToolError {
  fn into_response(self) -> Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    // Create a placeholder Axum response.
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the response.
    response.extensions_mut().insert(self);

    response
  }
}

// impl std::error::Error for ToolError {}
