use std::sync::{Arc, Mutex};
use axum::body::Bytes;
use axum::Extension;
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::extract::ws::Message::Ping;
use axum::response::IntoResponse;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tracing::{debug, error};
use crate::model::app_state::{AppState, OnlineUser, SocketEvent};
use crate::model::auth::Profile;
use crate::service;
use crate::service::{task_create, task_query, task_update};
use crate::ws::message::task::{SearchTaskTag, TaskMessage, UpdateTaskTag};

pub async fn task_handler(
  ws: WebSocketUpgrade,
  // TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
  Extension(profile): Extension<Arc<Mutex<Profile>>>,
  State(state): State<AppState>,
) -> impl IntoResponse {
  ws.on_upgrade(|socket| task_socket(socket, profile, state))
}


async fn task_socket(socket: WebSocket, profile_arc: Arc<Mutex<Profile>>, app_state: AppState) {
  // TODO: 拒绝访问, 如果userType不是VerifiedUser.
  let AppState {sender: broadcast_sender, private_sockets, official_broadcast ,.. } = app_state.clone();
  let (mut socket_sender, mut socket_receiver) = socket.split();
  let (message_tx, mut message_rx) = mpsc::channel::<SocketEvent>(150);
  let mut broadcast_receiver = broadcast_sender.subscribe();

  let mut task_broadcast_receiver = official_broadcast.get("task").unwrap().subscribe();

  debug!("into task socket!");
  /// init here
  let online_user: OnlineUser;
  {
    let profile = profile_arc.lock().unwrap();
    online_user = OnlineUser {
      user_id: profile.user_id.clone(),
      user_name: profile.user_name.clone(),
      telephone: profile.user_phone.clone(),
      sender: message_tx.clone(),
    };
    private_sockets.insert(profile.user_id.clone(), online_user.clone());
  }


  let tx1 = message_tx.clone();
  tokio::spawn(async move {
    while let Ok(msg) = task_broadcast_receiver.recv().await {
      debug!("接收到task广播消息, 传入广播事件! message: {:?}", msg);
      match tx1.send(SocketEvent::Broadcast(Bytes::from(msg))).await {
        Ok(_) => {debug!("传入广播事件成功! 等待处理...")}
        Err(e) => {
          error!("传入广播事件失败!");
          // break;
        }
      }
    }
  });

  // 任务2：接收客户端原始消息
  let tx2 = message_tx.clone();
  tokio::spawn(async move {
    while let Some(Ok(msg)) = socket_receiver.next().await {
      debug!("接收到客户端消息, 传入客户端事件! message: {:?}", msg);
      match tx2.send(SocketEvent::Client(msg)).await {
        Ok(_) => {debug!("传入客户端事件成功! 等待处理...")}
        Err(e) => {
          error!("传入客户端事件失败!");
          // break;
        }
      }
    }
  });

  // 任务3：心跳定时器
  let tx3 = message_tx.clone();
  let online_user_id = online_user.user_id.clone();
  tokio::spawn(async move {
    let duration = 30;
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(duration));
    loop {
      interval.tick().await;
      if tx3.send(SocketEvent::Heartbeat).await.is_err() {
        error!("心跳检测失败,用户{}断开连接 !", online_user_id);
        // break;
      }
    }
  });


  /// 发送消息统一使用message_tx
  let tip_message = format!("用户{}连接", online_user.user_id);
  message_tx.send(SocketEvent::Client(Message::Text(Utf8Bytes::from(tip_message)))).await.unwrap();


  // 主消息处理循环
  while let Some(event) = message_rx.recv().await {
    debug!("进入事件循环, 当前接收到事件: {:?}", event);
    match event {
      SocketEvent::Broadcast(msg) => {
        if let Err(e) = socket_sender.send(Message::Text(Utf8Bytes::try_from(msg).unwrap())).await {
          debug!("广播发送失败: {}", e);
          debug!("websocket连接将要关闭!");
          // break;
        }
      },
      SocketEvent::Client(Message::Text(text)) => {
        // 返回确认消息 (告诉客户端,命令已经收到, 正在处理)
        let ack_message = format!("ACK:{:?}", text);
        let _ = socket_sender.send(Message::Text(Utf8Bytes::from(ack_message))).await;
        match TaskMessage::from_str(&text) {
          Ok(task_message) => {
            debug!("收到命令: {:?}", task_message);
            // 根据 action、id、tag 处理业务逻辑
            match task_message {
              TaskMessage::Update {id, tag} => {
                debug!("更新任务: id={}, tag={:?}", id, tag);
                let status = convert_status_from_update_tag(tag);
                task_update::service(profile_arc.clone(), id.clone(), status, app_state.clone());
              }
              TaskMessage::Search {id, tag} => {
                debug!("搜索任务: id={}, tag={:?}", id, tag);
                match tag {
                  SearchTaskTag::Only => {
                    task_query::service(profile_arc.clone(), Some(id.clone()), None, app_state.clone());
                  }
                  SearchTaskTag::List => {
                    task_query::service(profile_arc.clone(), None, Some("idle".to_string()), app_state.clone());
                  }
                }
              }
              _ => {} // 其他 action...
            }

          }
          Err(e) => {
            debug!("非标准命令task命令: {}", e);
            // let _ = socket.send(Message::Text(format!("ERROR:{}", e))).await;
          }
        }

      },
      SocketEvent::Heartbeat => {
        if let Err(e) = socket_sender.send(Ping(Bytes::new())).await {
          debug!("心跳发送失败: {}", e);
          break;
        }
      },
      _ => {} // 忽略其他消息类型
    }
  }
  // 清理逻辑
  debug!("WebSocket连接关闭");
}

fn convert_status_from_update_tag(tag: UpdateTaskTag) -> String {
  match tag {
    UpdateTaskTag::Pend => {"pending".to_string()}
    UpdateTaskTag::Done => {"done".to_string()}
    UpdateTaskTag::Idle => {"idle".to_string()}
  }

}


