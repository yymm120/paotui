#![allow(unused)]
use crate::core::error::Result;
use crate::model::event::app_started_result::AppStartedResult;
use crate::model::http::init_response::InitResponse;
use crate::plugin::http::{http_client, q};
use log::log;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tauri::async_runtime::block_on;
use tauri::{Manager, Wry};
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_store::{Store, StoreExt};

pub struct AppData {
  pub token: String,
}

pub fn setup_for_initial(app: &mut tauri::App) -> Result<()> {

  log::info!("============= Setting up initial ===============");
  let store = app.store("store.json").expect("Store initial setup failure");
  let token = store.get("token").unwrap_or(Value::default()).to_string();
  store.close_resource();

  // 获取请求结果
  // case1: 服务端未就绪, 打开错误提示页面.
  // case2: 服务端就绪, 当前token无效, 根据response存储新token, 打开登录页面
  // case3: 服务端就绪, 当前token有效, 打开应用

  app.manage(Mutex::new(AppData {
    token,
  }));
  log::info!("state initialized");

  Ok(())
}
