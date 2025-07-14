use serde::{Deserialize, Serialize};
use crate::plugin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitResponse {
  user: User,
  initial_page: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
  user_id: String,
  user_type: String,
  phone_number: String,
}

impl From<plugin::error::Error> for crate::core::error::Error {
  fn from(val: plugin::error::Error) -> Self {
    crate::core::error::Error::PluginError(val)
  }
}