use crate::core::error::Result;
use crate::plugin::http::http_client;
use serde_json::json;
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri_plugin_http::reqwest::Client;
use tauri_plugin_store::{Store, StoreExt};

struct AppData {
    welcome_message: &'static str,
    /// reqwest client
    query_client: Client,
    /// 存储, 后台关闭, 前台开启
    store: Arc<Store<Wry>>,
}

pub fn setup_for_initial(app: &mut tauri::App) -> Result<()> {
    let store = app.store("store.json")?;

    let token = store.get("token").expect("Failed to get value from store");

    println!("{}", token);

    // store.close_resource();

    let query_client = http_client(token.as_str().unwrap_or(""))?;

    app.manage(AppData {
        welcome_message: "Welcome to Tauri!",
        query_client: query_client,
        store: store,
    });

    Ok(())
}
