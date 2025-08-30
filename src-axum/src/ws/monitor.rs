use axum::body::Bytes;
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::extract::ws::Message::Ping;
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use tracing::debug;
use crate::model::app_state::AppState;

// 定义统一消息枚举
#[derive(Debug)]
enum SocketEvent {
  Broadcast(Bytes),  // 广播消息
  Client(Message),    // 客户端消息
  Heartbeat,          // 心跳事件
}

pub async fn monitor_handler(
  ws: WebSocketUpgrade,
  State(state): State<AppState>,
) -> impl IntoResponse {
  ws.on_upgrade(|socket| monitor_socket(socket, state))
}


async fn monitor_socket(socket: WebSocket, state: AppState) {
  let AppState {sender: broadcast_sender, private_sockets, .. } = state;
  let (mut socket_sender, mut socket_receiver) = socket.split();

  let (message_tx, mut message_rx) = mpsc::channel::<SocketEvent>(200);

  let mut broadcast_receiver = broadcast_sender.subscribe();

  let tx1 = message_tx.clone();
  tokio::spawn(async move {
    while let Ok(msg) = broadcast_receiver.recv().await {
      // 非阻塞检查通道是否已关闭
      if tx1.send(SocketEvent::Broadcast(Bytes::from(msg))).await.is_err() {
        break;
      }
    }
  });

  // 任务2：接收客户端原始消息
  let tx2 = message_tx.clone();
  tokio::spawn(async move {
    while let Some(Ok(msg)) = socket_receiver.next().await {
      if tx2.send(SocketEvent::Client(msg)).await.is_err() {
        break;
      }
    }
    // 客户端断开时自动触发通道关闭
  });

  // 任务3：心跳定时器
  let tx3 = message_tx;
  tokio::spawn(async move {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
      interval.tick().await;
      if tx3.send(SocketEvent::Heartbeat).await.is_err() {
        break;
      }
    }
  });

  debug!("into monitor socket!");
  let users = private_sockets.iter()
    .map(|entry| entry.value().clone())
    .collect::<Vec<_>>();

  let _ = socket_sender.send(Message::Text(
    Utf8Bytes::from(serde_json::json!({
            "type": "user_list",
            "users": users
        }).to_string())
  )).await;


  // 主消息处理循环
  while let Some(event) = message_rx.recv().await {
    match event {
      SocketEvent::Broadcast(msg) => {
        if let Err(e) = socket_sender.send(Message::Text(Utf8Bytes::try_from(msg).unwrap())).await {
          eprintln!("广播发送失败: {}", e);
          break;
        }
      },
      SocketEvent::Client(Message::Text(text)) => {
        if text == "get_users" {
          // 处理业务逻辑...
        }
      },
      SocketEvent::Heartbeat => {
        if let Err(e) = socket_sender.send(Ping(Bytes::new())).await {
          eprintln!("心跳发送失败: {}", e);
          break;
        }
      },
      _ => {} // 忽略其他消息类型
    }
  }
  // 清理逻辑
  eprintln!("WebSocket连接关闭");
}
