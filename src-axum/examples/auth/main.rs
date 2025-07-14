#![allow(unused)]
fn main() {
  // println!("hello");
}

#[cfg(test)]
mod test {
  use anyhow::Result;
  use axum::Json;
  use reqwest::header::AUTHORIZATION;
  use serde::Serialize;

  #[tokio::test]
  async fn login() -> Result<()> {
    println!("第一次启动应用, 发起初始请求, 获得token");
    // 1. 启动应用, 第一次发起请求 api/_ 初始化应用请求
    let res = reqwest::Client::new()
      .get("http://localhost:3000/api/_")
      .send()
      .await
      .expect("没有拿到response.");

    let headers = res.headers().clone();
    let text = res.text().await?;

    println!("res: {:#}", serde_json::to_string_pretty(&text)?);

    let token = headers
      .get(AUTHORIZATION)
      .expect("failed to get token!")
      .to_str()?;
    println!("token: {}", token);

    println!();
    println!("第二次发起请求, 获取短信验证码");

    #[derive(Serialize)]
    struct CodeBody {
      phone_number: String,
    }
    // 2. 用户拿到token, 开始获取验证码
    let res = reqwest::Client::new()
      .post("http://localhost:3000/api/code")
      .json(&CodeBody {
        phone_number: "17815349593".to_string(),
      })
      .send()
      .await
      .expect("没有拿到response.");

    println!("{}", res.status().as_u16()); // 405

    let text = res.text().await?;

    println!("res: {:#}", serde_json::to_string_pretty(&text)?);

    println!();
    println!("第三次发起请求, 登录");
    #[derive(Serialize)]
    struct LoginBody {
      code: String,
    }
    // 2. 用户拿到验证码, 开始登录
    let res = reqwest::Client::new()
      .post("http://localhost:3000/api/login")
      .json(&LoginBody {
        code: "1234".to_string(),
      })
      .send()
      .await
      .expect("没有拿到response.");

    let text = res.text().await?;

    println!("res: {:#}", serde_json::to_string_pretty(&text)?);
    Ok(())
  }
}
