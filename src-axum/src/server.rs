#![allow(unused)]

mod api;
mod error;
mod model;
mod middleware;
mod service;
mod utils;
mod ws;
// mod lib;
mod query;

use crate::api::auth;
use crate::model::app_state::AppState;
use crate::utils::db::{Db, new_db_pool};
use axum::{Router, response::Html, routing::get};
use tracing::info;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{EnvFilter, prelude::*, util::SubscriberInitExt};

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
    // .with_f
    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%z".into()))
    .with_ansi(true)
    .with_level(true)
    .with_env_filter(EnvFilter::from_default_env())
    .init();

  let db = new_db_pool("postgres://postgres:postgres@localhost/paotui")
    .await
    .expect("Failed to create database pool");

  let app_state = AppState { db };
  // 路由
  let app = app_routes(app_state).await;

  // .merge(utils::tools())
  // .with_state(db.clone());

  // 地址
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  info!("Listening on http://0.0.0.0:3000");
  axum::serve(listener, app).await.unwrap();
}

async fn app_routes(app_state: AppState) -> Router {
  Router::new()
    .merge(auth::routes(app_state.clone()))
    .with_state(app_state)
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
