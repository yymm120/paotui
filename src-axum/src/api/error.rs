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

  #[error("用户id: {0} 未经授权, 无法访问!")]
  Unauthorized(String)
}


impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    debug!("{:<12} - model::Error {self:?}", "INTO_RES");

    // 根据错误类型返回不同的状态码和消息
    let (status, message) = match self {
      ApiError::ValidationError(err) => {
        let msg = format!("Input validation error: {}", err);
        (StatusCode::BAD_REQUEST, msg)
      }
      ApiError::AxumFormRejection(err) => {
        (StatusCode::BAD_REQUEST, err.to_string())
      }
      ApiError::Unauthorized(err) => {
        let msg = format!("用户id: {0} 未经授权, 无法访问!", err);
        (StatusCode::UNAUTHORIZED, msg)
      }
      _ => {
        (StatusCode::INTERNAL_SERVER_ERROR, "OOPS! 发生未知错误!".to_string())
      }
    };

    // 创建响应并插入错误到扩展（如需后续处理）
    let mut response = (status, message).into_response();
    // response.extensions_mut().insert(self);
    response
  }
}

// impl std::error::Error for ApiError {}
