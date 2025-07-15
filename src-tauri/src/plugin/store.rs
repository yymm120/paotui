use crate::plugin::error::Result;
use serde_json::json;
use tauri::App;
use tauri_plugin_store::StoreExt;

pub fn setup_store(app: &mut App) -> Result<()> {
  // 创建一个新的存储空间或加载已有的存储空间
  // 此操作同时会将存储空间注册到应用的资源表中
  // 因此后续无论是从Rust还是JavaScript发起的store调用,会重用相同的store
  let store = app.store("store.json")?;

  // 请注意，值必须是 serde_json：：Value 实例，
  // 否则，它们将与 JavaScript 绑定不兼容。
  store.set("some-key", json!({ "value": 5 }));

  // Get a value from the store.
  let value = store
    .get("some-key")
    .expect("Failed to get value from store");
  log::info!("{}", value); // {"value":5}

  // Remove the store from the resource table
  store.close_resource();
  Ok(())
}
