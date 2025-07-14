use std::sync::OnceLock;
pub use tokio::{
  runtime::{Handle as TokioHandle, Runtime as TokioRuntime},
  sync::{
    Mutex, RwLock,
    mpsc::{Receiver, Sender, channel},
  },
  task::JoinHandle as TokioJoinHandle,
};

static RUNTIME: OnceLock<TokioRuntime> = OnceLock::new();

pub fn default_runtime() -> &'static TokioRuntime {
  let runtime = TokioRuntime::new().unwrap();
  RUNTIME.get_or_init(|| runtime)
  // let handle = runtime.handle();
}
