use std::net::SocketAddr;
use crate::model::app_state::{AppState, OnlineUser};
use axum::extract::{State, WebSocketUpgrade, ws, Query};
use axum::http::Version;
use axum::extract::connect_info::ConnectInfo;
use axum::response::Response;
use std::time::Duration;
use axum::{debug_handler, extract, middleware, Router};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::routing::{any, get, post};
use fake::Fake;
use fake::faker::name::zh_cn::Name;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::{broadcast, mpsc};
use tokio::sync::broadcast::Sender;
use tracing::debug;
use crate::middleware::auth_check_middleware;
use crate::ws::monitor::monitor_handler;
use crate::ws::task_handle::task_handler;

pub mod check;
pub mod error;
pub mod monitor;
pub mod task_handle;
pub mod message;

pub fn routes(app_state: AppState) -> Router<AppState> {
  Router::new().nest(
    "/ws",
    Router::new()
      // .route("/", any(ws_handler))
      .route("/task", any(task_handler))
      .route("/monitor", any(monitor_handler))
      .layer(middleware::from_fn_with_state(
        app_state,
        auth_check_middleware::auth_check,
      )),
  )
}


#[derive(Deserialize, Debug)]
pub struct ConnectQuery {
  pub user_id: String,
}
//
// #[debug_handler]
// pub async fn ws_handler(
//   websocket_upgrade: WebSocketUpgrade,
//   version: Version,
//   State(app_state): State<AppState>,
//   Query(query): Query<ConnectQuery>,
//   ConnectInfo(addr): ConnectInfo<SocketAddr>,
// ) -> Response {
//   debug!("into ws_handler, {:?}", query);
//   websocket_upgrade.protocols(["chat"]).on_upgrade(move |mut socket| handle_chat_socket(socket, addr, query, app_state))
//   // websocket_upgrade.protocols(["task"]).on_upgrade(move |mut socket| handle_socket(socket, addr, query, app_state))
// }
//
// async fn handle_chat_socket(mut socket: WebSocket, who: SocketAddr, query: ConnectQuery, app_state: AppState) {
//   let AppState { db, app_identify, private_sockets, sender: broadcast_sender  } = app_state;
//   let ConnectQuery {user_id, ..} = query;
//   debug!("into handle_chat_socket, {:?}", who);
//   let (tx, mut rx) = mpsc::channel::<String>(25);
//   let user = OnlineUser {
//     user_id: user_id.clone(),
//     user_name: Name().fake(),
//     sender: tx,
//   };
//   private_sockets.insert(user_id.clone(), user);
//   debug!("private_sockets: {:?}", private_sockets);
//
//   // let _ = broadcast_sender.send(UserEvent::UserJoined { user: user_info });
//   // let _ = broadcast_sender.send(UserEvent::UserLeft { user_id });
//
//   let (mut sender, mut receiver) = socket.split();
//   let id = user_id.clone();
//
//   tokio::spawn(async move {
//     debug!("RX task started for {}", id.clone());
//     while let Some(msg) = rx.recv().await {
//       debug!("RX received: {:?}", msg);
//       if sender.send(Message::Text(Utf8Bytes::from(msg))).await.is_err() {
//         debug!("WebSocket closed, stopping RX task");
//         break;
//       }
//     }
//     debug!("RX task ended for {}", id.clone());
//   });
//
//   while let Some(Ok(msg)) = receiver.next().await {
//     // 这里可以处理接收到的消息
//     // 例如解析消息内容并转发给特定用户
//     debug!("into receiver: {:?}", msg);
//     if let Ok(text) = msg.into_text() {
//       if let Some((target_user, message)) = text.split_once(':') {
//         debug!("into split_once: {:?}, {:?}", target_user, message);
//         if let Some(tx) = private_sockets.get(target_user) {
//           debug!("tx send: {:?}", message.to_string());
//
//           let _ = tx.sender.send(message.to_string()).await;
//         }
//       }
//     }
//   }
//
//   // 连接关闭时移除用户
//   private_sockets.remove(&user_id);
// }
// async fn handle_socket(mut socket: WebSocket, who: SocketAddr, query: ConnectQuery, app_state: AppState) {
//   let AppState { db, sender, app_identify, private_sockets  } = app_state;
//   let ConnectQuery {user_id, ..} = query;
//   debug!("into handle_socket, {:?}", who);
//   let (tx, mut rx) = mpsc::channel::<String>(25);
//   // private_sockets.insert(user_id.clone(), tx);
//   // debug!("private_sockets: {:?}", private_sockets);
//
//   let mut receiver = sender.subscribe();
//   loop {
//     tokio::select! {
//       res = socket.recv() => match res {
//         Some(Ok(ws::Message::Text(s))) => {
//           let _ = sender.send(s.to_string());
//         }
//         Some(Ok(ws::Message::Ping(ping))) => {
//           let _ = socket.send(ws::Message::Pong(ping)).await;
//         }
//         Some(Ok(_)) => {}
//         Some(Err(e)) => {
//           tracing::debug!("client error: {e}");
//           break;
//         }
//         None => break,
//       },
//       res = receiver.recv() => match res {
//         Ok(msg) => if let Err(e) = socket.send(ws::Message::Text(format!("aaaaa {}", msg).into())).await {
//           tracing::debug!("send error: {e}");
//           break;
//         },
//         Err(broadcast::error::RecvError::Lagged(_)) => {
//           tracing::warn!("client lagging behind, skipping messages");
//         }
//         Err(_) => break,
//       },
//       _ = tokio::time::sleep(Duration::from_secs(60 * 30)) => { // 需要更新
//         tracing::debug!("client inactive for 60s, disconnecting");
//         break;
//       }
//     }
//   }
// }
//
