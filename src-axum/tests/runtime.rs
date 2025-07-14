#![allow(unused)]
use futures::TryFutureExt;
use std::sync::OnceLock;
use tokio::try_join;
pub use tokio::{
  runtime::{Handle as TokioHandle, Runtime as TokioRuntime},
  sync::{
    Mutex, RwLock,
    mpsc::{Receiver, Sender, channel},
  },
  task::JoinHandle as TokioJoinHandle,
};

static RUNTIME: OnceLock<GlobalRuntime> = OnceLock::new();

pub struct GlobalRuntime {
  runtime: Option<TokioRuntime>,
  handle: TokioHandle,
}

pub fn default_runtime() -> GlobalRuntime {
  let runtime = TokioRuntime::new().unwrap();
  let handle = runtime.handle().clone();
  GlobalRuntime {
    runtime: Some(runtime),
    handle,
  }
}
pub fn block_on<F: Future>(task: F) -> F::Output {
  match TokioHandle::try_current() {
    Ok(handle) => {
      // 已在 runtime 内，使用 block_in_place 避免嵌套
      // 当前runtime必须是多线程才能使用
      tokio::task::block_in_place(|| handle.block_on(task))
    }

    Err(_) => {
      // 无 runtime，使用全局 runtime
      let runtime = RUNTIME.get_or_init(default_runtime);
      runtime.handle.block_on(task)
    }
  }
  // let runtime = RUNTIME.get_or_init(default_runtime);
  // runtime.handle.block_on(task)
}
