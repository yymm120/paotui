#![allow(unused)]
use axum::Router;
use axum::routing::{get, post};

pub fn routes() -> Router {
  Router::new()
    .route("ws/ping", get(ping))
    .route("ws/pong", get(ping))
  // .with_state(mm)
}

async fn ping() {}

pub fn pong() {}
