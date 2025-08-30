// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use api::auth::{auth_code, auth_init, auth_login};
use core::setup::setup_for_initial;
use plugin::store::setup_store;
use tauri_plugin_store::StoreExt;

mod api;
mod constant;
mod core;
mod event;
mod model;
mod plugin;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .plugin(tauri_plugin_sql::Builder::new().build())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_websocket::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_txmap::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_geolocation::init())
    .setup(|app| {
      // setup_store(app)?;
      setup_for_initial(app)?;
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![auth_init, auth_code, auth_login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
