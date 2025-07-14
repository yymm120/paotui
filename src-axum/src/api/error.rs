use crate::service;
use crate::utils;
use axum::extract::rejection::{FormRejection, JsonRejection, PathRejection};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use tracing::debug;

// Result
pub type Result<T> = core::result::Result<T, ApiError>;

#[derive(Error, Debug, strum_macros::AsRefStr)]
pub enum ApiError {
  // #[error(transparent)]
  // AuthError(&'static str),
  #[error(transparent)]
  ServiceError(#[from] service::error::ServiceError),

  #[error(transparent)]
  ToolError(#[from] utils::error::ToolError),

  #[error("Authentication failed: {0}")]
  AuthTokenMissError(String),

  #[error("jsonwebtoken error: {0}")]
  AuthTokenError(#[from] jsonwebtoken::errors::Error),

  #[error("sqlx error: {0}")]
  DataBaseError(#[from] sqlx::error::Error),

  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),

  #[error(transparent)]
  AxumFormRejection(#[from] FormRejection),

  #[error(transparent)]
  AxumPathRejection(#[from] PathRejection),

  #[error(transparent)]
  AxumJsonRejection(#[from] JsonRejection),
}

// impl From<jsonwebtoken::errors::Error> for ApiError {
//   fn from(val: jsonwebtoken::errors::Error) -> Self {
//     ApiError::AuthTokenError(val)
//   }
// }
//
// impl From<sqlx::error::Error> for ApiError {
//   fn from(val: sqlx::error::Error) -> Self {
//     ApiError::DataBaseError(val)
//   }
// }

// impl From<FormRejection> for ApiError {
//   fn from(val: FormRejection) -> Self {
//     ApiError::AxumFormRejection(val)
//   }
// }

// impl Display for ApiError {
//   fn fmt(&self, fmt: &mut Formatter) -> core::result::Result<(), core::fmt::Error> {
//     write!(fmt, "{self:?}")
//   }
// }

impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    // Create a placeholder Axum response.
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the response.
    // response.extensions_mut().insert(self);

    // match self {
    //   ApiError::ValidationError(_) => {
    //     let message = format!("Input validation error: [{self}]").replace('\n', ", ");
    //     (StatusCode::BAD_REQUEST, message)
    //   }
    //   ApiError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
    // }
    //   .into_response();

    response
  }
}

// impl std::error::Error for ApiError {}
