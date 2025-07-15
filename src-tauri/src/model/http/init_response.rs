#![allow(unused)]
use crate::model::event::app_started_result::AppStartedResult;
use crate::plugin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitResponse {
  pub status: bool,
  pub user: User,
  // initial_page: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  pub user_id: String,
  pub user_type: String,
  pub phone_number: String,
}

impl core::fmt::Display for InitResponse {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}
