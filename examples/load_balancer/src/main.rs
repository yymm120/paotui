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
    session: &mut Session,
    ctx: &mut Self::CTX,
  ) -> Result<Box<HttpPeer>> {
    const MAX_SLEEP: Duration = Duration::from_secs(5);

    if ctx.retries > 0 {
      // simple example of exponential backoff with a max of 10s
      let sleep_ms = std::cmp::min(Duration::from_millis(u64::pow(5, ctx.retries)), MAX_SLEEP);
      info!("sleeping for ms: {sleep_ms:?}");
      tokio::time::sleep(sleep_ms).await;
    }
    if let Some(host) = session.req_header().headers.get("host") {
      if host == "tauri.localhost" {
        let mut peer = HttpPeer::new(("192.168.10.107", 5173), false, "".into());
        peer.options.connection_timeout = Some(Duration::from_millis(100));
        return Ok(Box::new(peer));
      }
    }
    Err(Error::new_str("aaaaaa"))
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

  let mut my_proxy = pingora_proxy::http_proxy_service(&my_server.configuration, BackoffRetryProxy);
  my_proxy.add_tcp("0.0.0.0:80");
  // let cert_path = format!("{}/tests/keys/server.crt", env!("CARGO_MANIFEST_DIR"));
  // let key_path = format!("{}/tests/keys/key.pem", env!("CARGO_MANIFEST_DIR"));

  // mkcert -key-file key.pem -cert-file cert.pem example.com *.example.com
  // let cert_path = "cert.pem";
  // let key_path = "key.pem";
  //
  // let mut tls_settings =
  //     pingora_core::listeners::tls::TlsSettings::intermediate(&cert_path, &key_path).unwrap();
  // tls_settings.enable_h2();
  // my_proxy.add_tls_with_settings("0.0.0.0:443", None, tls_settings);
  //
  my_server.add_service(my_proxy);
  my_server.run_forever();
}
