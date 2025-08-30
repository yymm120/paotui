#![allow(unused)]
use serde::Serialize;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, QueryError>;

#[derive(Debug, Error)]
pub enum QueryError {
  #[error("Error message: {0}")]
  Error(String),

  #[error(transparent)]
  DbError(#[from] sqlx::Error),
}
