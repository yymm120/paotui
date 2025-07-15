#![allow(unused)]
use crate::model::http::init_response::InitResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppStartedResult {
  user: User,
  initial_page: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
  user_id: String,
  user_type: String,
  phone_number: String,
}

impl From<InitResponse> for AppStartedResult {
  fn from(val: InitResponse) -> Self {
    Self {
      user: User {
        user_id: "".to_string(),
        user_type: "".to_string(),
        phone_number: "".to_string(),
      },
      initial_page: "".to_string(),
    }
  }
}

impl core::fmt::Display for AppStartedResult {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}
