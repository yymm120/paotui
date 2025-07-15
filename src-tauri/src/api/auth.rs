#![allow(unused)]

use crate::core::setup::AppData;
use crate::model::http::init_response::InitResponse;
use crate::plugin::http::{http_client, q, LoginData, QueryBody};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::http::header::AUTHORIZATION;
use tauri::http::{HeaderValue, Method};
use tauri::{Manager, State};
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_store::StoreExt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
#[tauri::command]
pub async fn auth_init(
  app_handle: tauri::AppHandle,
  state: State<'_, Mutex<AppData>>,
) -> Result<InitResponse, Error> {
  let client = http_client(state.clone()).unwrap();
  let query_response = client
    .clone()
    .method(Method::GET)
    .query::<InitResponse>("/api/_")
    .await
    .unwrap();
  let init_data = query_response.body;
  let response_token = query_response
    .header_map
    .get(AUTHORIZATION)
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();

  let client_token = client
    .header_map
    .get(AUTHORIZATION)
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();

  log::debug!("response token: {}", response_token.clone());

  if response_token != client_token {
    let store = app_handle.store("token.json").unwrap();
    let store_token = store.get("token").unwrap_or(Value::default()).to_string();
    store.set("token", response_token.clone());
    store.close_resource();
    {
      let mut state = state.lock().unwrap();
      state.token = response_token.clone();
    }
  }

  Ok(init_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
  pub status: bool,
}
#[tauri::command(rename_all = "snake_case")]
pub async fn auth_login(
  user_phone: String,
  code: String,
  state: State<'_, Mutex<AppData>>,
) -> Result<LoginResult, Error> {
  let client = http_client(state).unwrap();

  let query_response = client
    .method(Method::POST)
    .body(QueryBody::AuthLogin(LoginData {
      user_phone: user_phone.clone(),
      code,
    }))
    .query::<LoginResult>("/api/login")
    .await
    .unwrap();

  let login_result = query_response.body;

  Ok(login_result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResult {
  pub code: String,
}

#[tauri::command]
pub async fn auth_code(state: State<'_, Mutex<AppData>>) -> Result<CodeResult, Error> {
  let client = http_client(state).unwrap();

  let query_response = client
    .method(Method::GET)
    .query::<CodeResult>("/api/code")
    .await
    .unwrap();

  let code_result = query_response.body;
  Ok(code_result)
}

#[cfg(test)]
mod tests {
  use crate::model::http::init_response::InitResponse;
  use crate::plugin::http::{http_client, q};
  use tauri::async_runtime::block_on;
  use tauri::http::header::AUTHORIZATION;
  use tauri_plugin_http::reqwest::Client;
}
