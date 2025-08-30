#![allow(unused)]

mod api;
mod error;
mod middleware;
mod service;
mod utils;
mod ws;
// mod lib;
// mod query;
mod model;

use crate::api::{auth, profile, task};
use crate::model::app_state::{AppIdentify, AppState, OfficialBroadcastMap, PrivateSocketMap };
use crate::utils::db::{Db, new_db_pool};
// use crate::ws::ws_handler;
use axum::extract::{State, WebSocketUpgrade};
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, Version};
use axum::routing::any;
use axum::{Router, response::Html, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use rustls::ServerConfig;
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{EnvFilter, prelude::*, util::SubscriberInitExt};
use crate::ws::monitor::monitor_handler;
use crate::ws::task_handle::task_handler;

#[tokio::main]
async fn main() {
  application().await
}

async fn application() {
  // 日志
  tracing_subscriber::fmt()
    // .with_target(true)       // 显示日志目标（模块路径）
    .with_file(true) // 显示源文件路径 ← 新增
    .with_line_number(true) // 显示行号 ← 新增
    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%z".into()))
    .with_ansi(true)
    .with_level(true)
    .with_env_filter(EnvFilter::from_default_env())
    .init();

  rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

  let db = new_db_pool("postgres://postgres:postgres@localhost/paotui")
    .await
    .expect("Failed to create database pool");

  let app_state = AppState {
    sender: broadcast::channel::<String>(32).0,
    db,
    app_identify: AppIdentify::DeliveryApp,
    private_sockets: PrivateSocketMap::default(),
    official_broadcast: OfficialBroadcastMap::default(),
  };
  app_state.official_broadcast
    .insert("task".to_string(), broadcast::channel::<String>(64).0);
  // 路由
  let app = app_routes(app_state).await;



  let config = RustlsConfig::from_pem_file(
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cert").join("cert.pem"),
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("cert").join("key.pem"),
  )
  .await
  .unwrap();

  let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
  info!("Listening on https://0.0.0.0:4000");
  let mut server = axum_server::bind_rustls(addr, config);
  server.http_builder().http2().enable_connect_protocol();
  server.serve(app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn app_routes(app_state: AppState) -> Router {
  let cors = CorsLayer::new()
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_origin(Any);

  Router::new()
    .merge(ws::routes(app_state.clone()))
    .merge(api::routes(app_state.clone()))
    .with_state(app_state)
    .layer(cors)
}

#[cfg(test)]
mod tests {
  use anyhow::{Context, Result};
  use tracing::info;
  // use serial_test::serial;

  use crate::application;

  #[tokio::test]
  async fn start_server() {
    application().await
  }
}
