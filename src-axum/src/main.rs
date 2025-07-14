#![allow(unused)]
use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
  // build our application with a single route
  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

  println!("Listening on http://0.0.0.0:3000");
  axum::serve(listener, app).await.unwrap();
}
