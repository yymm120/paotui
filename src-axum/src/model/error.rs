#![allow(unused)]

use std::fmt::{Display, Formatter};
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, ModelError>;

#[derive(Debug, Error)]
pub enum ModelError {
  #[error("Error message: {0}")]
  Error(String),

  #[error(transparent)]
  DbError(#[from] sqlx::Error),

  #[error(transparent)]
  ValidationError(#[from] validator::ValidationErrors),

  #[error(transparent)]
  TokioTryCurrentRuntimeError(#[from] tokio::runtime::TryCurrentError),
}
