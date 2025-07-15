#![allow(unused)]

use crate::core::setup::AppData;
use crate::plugin::error::Result;
use itertools::Itertools;
use log::debug;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Duration;
use tauri::http::{HeaderName, Method, StatusCode};
use tauri::State;
use tauri_plugin_http::reqwest;
use tauri_plugin_http::reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tauri_plugin_http::reqwest::{Client, Response, Url};

pub const END_POINT_URL: &str = "http://127.0.0.1:3000";

pub fn q(base_url: Url, path: &str) -> Url {
  base_url.join(path).expect(
    format!(
      "拼接 url 发生意料之外的错误, base_url: {}, path: {}",
      END_POINT_URL, path
    )
    .as_str(),
  )
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginData {
  pub user_phone: String,
  pub code: String,
}

#[derive(Debug, Clone)]
pub enum QueryBody {
  AuthLogin(LoginData),
}

#[derive(Debug, Clone)]
pub struct QueryClient {
  pub origin_client: Client,
  pub url: Url,
  pub method: Method,
  pub header_map: HeaderMap,
  pub body: Option<QueryBody>,
}

pub struct QueryResponse<T> {
  pub url: Url,
  pub header_map: HeaderMap,
  pub status: StatusCode,
  pub cookies: HashMap<String, String>,
  pub content_length: u64,
  pub remote_addr: SocketAddr,
  pub body: T,
}

impl QueryClient {
  pub async fn query<R>(self, path: &str) -> Result<QueryResponse<R>>
  where
    R: DeserializeOwned + Debug,
  {
    let (client, method, query_body, header_map, base_url) = (
      self.origin_client,
      self.method,
      self.body,
      self.header_map,
      self.url,
    );
    let url = q(base_url, path);
    let req = client.request(method.clone(), url.clone()).build().unwrap();
    debug!(
      "请求 - url: {}, method: {}, body: {:?}",
      url, method, query_body
    );
    let res = client.execute(req).await.unwrap();
    let (url, header_map, status, cookies, content_length, remote_addr, res_body) = (
      res.url().clone(),
      res.headers().clone(),
      res.status(),
      res
        .cookies()
        .map(|c| (c.name().to_string(), c.value().to_string()))
        .collect::<HashMap<String, String>>(),
      res.content_length().unwrap_or(0),
      res.remote_addr().unwrap(),
      res.json::<R>().await.unwrap(),
    );

    debug!("响应 - response: {:?}", res_body);
    Ok(QueryResponse {
      url,
      header_map,
      status: Default::default(),
      cookies,
      content_length,
      remote_addr,
      body: res_body,
    })
  }

  pub fn body(mut self, body: QueryBody) -> QueryClient {
    self.body = Some(body);
    self
  }

  pub fn header(mut self, key: HeaderName, value: HeaderValue) -> QueryClient {
    self.header_map.append(key, value);
    self
  }

  pub fn method(mut self, method: Method) -> QueryClient {
    self.method = method;
    self
  }
}

pub fn http_client(state: State<'_, Mutex<AppData>>) -> Result<QueryClient> {
  let mut state_token = "".to_string();
  {
    let state = state.lock().unwrap();
    state_token = state.token.clone();
  }
  let mut header_map = HeaderMap::default();
  header_map.append(
    AUTHORIZATION,
    HeaderValue::from_str(format!("Basic {}", state_token).as_str()).unwrap(),
  );

  let client = Client::builder()
    .connect_timeout(Duration::from_secs(10))
    .default_headers(header_map.clone())
    .build()?;
  let query_client = QueryClient {
    origin_client: client,
    url: Url::from_str(END_POINT_URL).unwrap(),
    method: Method::GET,
    header_map,
    body: None,
  };
  Ok(query_client)
}
