use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::body::Bytes;
use axum::extract::ws::Message;
use dashmap::DashMap;
use serde::Serialize;
use crate::utils::db::Db;
use tokio::sync::{broadcast, mpsc};
use crate::model::table::customer_person_table::CustomerPersonTable;
use crate::model::table::delivery_person::DeliveryPerson;

#[derive(Clone, Debug)]
pub enum AppIdentify {
  DeliveryApp,
  TaskApp,
}

#[derive(Debug, Clone)]
pub enum SocketEvent {
  Broadcast(Bytes),   // 广播消息
  Client(Message),    // 客户端消息
  Heartbeat,          // 心跳事件
}

#[derive(Clone, Debug, Serialize)]
pub struct OnlineUser {
  pub user_id: String,
  pub user_name: String,
  pub telephone: String,
  #[serde(skip)]
  pub sender: mpsc::Sender<SocketEvent>
}

/// 私聊 mpsc 通道
pub type PrivateSocketMap = Arc<DashMap<String, OnlineUser>>;

/// 群组广播
pub type GroupBroadcast = Arc<DashMap<String, broadcast::Sender<String>>>;

/// 官方广播
pub type OfficialBroadcastMap = Arc<DashMap<String, broadcast::Sender<String>>>;




#[derive(Clone, Debug)]
pub struct AppState {
  pub sender: broadcast::Sender<String>,
  pub db: Db,
  pub app_identify: AppIdentify,
  pub private_sockets: PrivateSocketMap,
  pub official_broadcast: OfficialBroadcastMap,
}
