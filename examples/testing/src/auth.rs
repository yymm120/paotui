#![allow(unused)]

use std::time::Duration;
use reqwest::{Client, Url};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InitResponse {
  pub status: bool,
  pub user: User,
  initial_page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
  pub user_id: String,
  pub user_type: String,
  pub phone_number: String,
}

fn main(){}
pub const END_POINT_URL: &str = "http://127.0.0.1:3000";
pub fn q(path: &str) -> Url {
  let base_url = Url::parse(END_POINT_URL).unwrap();
  base_url.join(path).expect(
    format!(
      "拼接 url 发生意料之外的错误, base_url: {}, path: {}",
      END_POINT_URL, path
    )
      .as_str(),
  )
}

pub async fn request_init(query_client: Client) -> anyhow::Result<InitResponse> {
  let request_future = query_client.get(q("api/_")).send();

  let res = request_future.await?;
  let token = res.headers().get(AUTHORIZATION);
  println!("token: {:?}", token.unwrap());
  let json = res.json::<InitResponse>().await?;
  // let text = request_future.await.unwrap().text().await?;
  // println!("{}", text);
  // println!("response: {:?}", res);
  Ok(json)
}

pub fn http_client(token: &str) -> anyhow::Result<Client> {
  let mut header_map = HeaderMap::default();
  header_map.append(
    AUTHORIZATION,
    HeaderValue::from_str(format!("Basic {}", token).as_str()).unwrap(),
  );

  let client = reqwest::Client::builder()
    .connect_timeout(Duration::from_secs(10))
    .default_headers(header_map)
    .build()?;

  Ok(client)
}

#[cfg(test)]
mod tests {
  use crate::auth::{http_client, request_init};

  #[tokio::test]
  async fn init() -> anyhow::Result<()> {
    let client = http_client("")?;
    let res = request_init(client).await?;

    Ok(())
  }
}


















