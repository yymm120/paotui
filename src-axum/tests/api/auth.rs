use reqwest::{Client, Url};

#[test]
fn init_server() -> anyhow::Result<()> {
  let query_client = Client::builder().build()?;
  Ok(())
}

#[test]
fn transform_01() {}
