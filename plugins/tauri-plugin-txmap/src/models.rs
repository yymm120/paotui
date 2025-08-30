use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Location {
  pub lat: f64,
  pub lng: f64
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserInfo {
  avatar: Option<String>
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Task {
  pub id: String,
  pub status: String,
  pub start_position: Location,
  pub end_position: Location,
  pub send_info: Option<UserInfo>,
  pub receive_info: Option<UserInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FragmentRequest {
  pub headless: Option<bool>,
  pub close: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FragmentResponse {
  pub status: Option<String>,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TxMapOptions {
  pub tasks: Option<HashMap<String, Task>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WatchOptions {
  pub watch: Option<bool>,
  pub timeout: Option<u32>,
  pub rate: Option<u32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WatchResponse {
  pub status: Option<String>,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TxMapResponse {
  pub status: Option<String>,
  pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTaskPayload {
  pub task: Option<Task>,
  pub preview: Option<bool>,
}
#[derive(Serialize)]
pub struct ChannelPayload {
  pub options: TxMapOptions,
  pub channel: Channel,
}

#[derive(Serialize)]
pub struct WatchPayload {
  pub options: WatchOptions,
  pub channel: Channel,
}


#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClearTaskPayload {
  pub id: String
}

#[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct ClearChannelPayload {
  pub id: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(untagged)]
pub enum WatchEvent {
  Task(Task),
  Error(String),
}