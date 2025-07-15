use std::time::Duration;

use async_trait::async_trait;
use clap::Parser;

use log::info;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_core::{prelude::Opt, Error};
use pingora_proxy::{ProxyHttp, Session};


#[derive(Default)]
struct RetryCtx {
  pub retries: u32,
}

struct BackoffRetryProxy;

#[async_trait]
impl ProxyHttp for BackoffRetryProxy {
  type CTX = RetryCtx;
  fn new_ctx(&self) -> Self::CTX {
    Self::CTX::default()
  }

  async fn upstream_peer(
    &self,
    _session: &mut Session,
    ctx: &mut Self::CTX,
  ) -> Result<Box<HttpPeer>> {
    const MAX_SLEEP: Duration = Duration::from_secs(10);

    if ctx.retries > 0 {
      // simple example of exponential backoff with a max of 10s
      let sleep_ms =
        std::cmp::min(Duration::from_millis(u64::pow(10, ctx.retries)), MAX_SLEEP);
      info!("sleeping for ms: {sleep_ms:?}");
      tokio::time::sleep(sleep_ms).await;
    }
    let mut peer = HttpPeer::new(("10.0.0.1", 80), false, "".into());
    peer.options.connection_timeout = Some(Duration::from_millis(100));
    Ok(Box::new(peer))
  }

  fn fail_to_connect(
    &self,
    _session: &mut Session,
    _peer: &HttpPeer,
    ctx: &mut Self::CTX,
    e: Box<Error>,
  ) -> Box<Error> {
    ctx.retries += 1;
    let mut retry_e = e;
    retry_e.set_retry(true);
    retry_e
  }
}

// RUST_LOG=INFO cargo run --example backoff_retry -- --conf examples/conf.yaml

fn main() {
  env_logger::init();

  // read command line arguments
  let opt = Opt::parse();
  let mut my_server = Server::new(Some(opt)).unwrap();
  my_server.bootstrap();

  let mut my_proxy =
    pingora_proxy::http_proxy_service(&my_server.configuration, BackoffRetryProxy);
  my_proxy.add_tcp("0.0.0.0:6195");

  my_server.add_service(my_proxy);
  my_server.run_forever();
}