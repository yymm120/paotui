use crate::utils::jwt::{decode_handler, encode_handler};
use axum::Router;
use axum::routing::post;

mod crypt;
pub mod db;
pub mod error;
pub mod jwt;
pub mod runtime;
mod time;

pub fn tools() -> Router {
  Router::new().nest(
    "/tools",
    Router::new()
      .route("/jwt_encode", post(encode_handler))
      .route("/jwt_decode", post(decode_handler)),
  )
}
