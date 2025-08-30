/// 骑手端, 棋手不能创建和删除任务, 只能修改和搜索任务
use thiserror::Error;

// ---- 错误定义 ----
#[derive(Debug, Error)]
pub enum MessageError {
  #[error("消息格式必须为 `action:id:tag`")]
  InvalidFormat,
  #[error("未知的 Action 类型: {0}")]
  UnknownAction(String),
  #[error("{0} 不支持 Tag: {1}")]
  InvalidTag(String, String), // (action, invalid_tag)
}


#[derive(Debug, Clone)]
pub enum UpdateTaskTag {
  Pend,
  Done,
  Idle,
}

#[derive(Debug, Clone)]
pub enum SearchTaskTag {
  Only,
  List,
}
impl TaskMessage {
  pub fn from_str(s: &str) -> Result<Self, MessageError> {
    let mut parts = s.splitn(3, ':');
    let action = parts.next().ok_or(MessageError::InvalidFormat)?;
    let id = parts.next().ok_or(MessageError::InvalidFormat)?;
    let tag = parts.next().ok_or(MessageError::InvalidFormat)?;

    match action {
      "update-task" => {
        let tag = match tag {
          "pend" => UpdateTaskTag::Pend,
          "done" => UpdateTaskTag::Done,
          "idle" => UpdateTaskTag::Idle,
          _ => return Err(MessageError::InvalidTag(action.to_string(), tag.to_string())),
        };
        Ok(Self::Update {
          id: id.to_string(),
          tag,
        })
      }
      "search-task" => {
        let tag = match tag {
          "only" => SearchTaskTag::Only,
          "list" => SearchTaskTag::List,
          _ => return Err(MessageError::InvalidTag(action.to_string(), tag.to_string())),
        };
        Ok(Self::Search {
          id: id.to_string(),
          tag,
        })
      }
      _ => Err(MessageError::UnknownAction(action.to_string())),
    }
  }
}
#[derive(Debug, Clone)]
pub enum TaskMessage {
  Update {
    id: String,
    tag: UpdateTaskTag,
  },
  Search {
    id: String,
    tag: SearchTaskTag,
  },
  // 其他消息变体...
}

