#![allow(unused)]
use sqlx::{Pool, Postgres };
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;
use validator::{Validate, ValidateArgs, ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Error)]
enum CheckError {
  #[error("check error {0}")]
  ValidationError(#[from] ValidationErrors),
}
pub struct ValidateContext {
  user: UserTable,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let valid_claims = ValidClaims {
    sub: Uuid::new_v4().to_string(),
    exp: 0,
    iat: 0,
    phone: "123".to_string(),
  };

  let user = ValidateContext {
    user: UserTable {
      user_id: "def".to_string(),
      user_phone: "".to_string(),
    },
  };

  let res = valid_claims.validate_with_args(&user);

  println!(
    "{}",
    CheckError::ValidationError(res.clone().err().unwrap())
  );

  println!("{}", res.clone().err().unwrap());
  let ValidationErrors(v) = res.err().unwrap();
  let vv = v.get("__all__").unwrap();
  match vv {
    ValidationErrorsKind::Struct(_) => {
      println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    }
    ValidationErrorsKind::List(_) => {
      println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
    }
    ValidationErrorsKind::Field(aa) => {
      println!("cccccccccccccccccccccccccccccccc")
    }
  }
  println!("hello");
  Ok(())
}

type Db = Pool<Postgres>;

struct UserTable {
  user_id: String,
  user_phone: String,
}

#[derive(Debug, Clone, Validate)]
#[validate(context = ValidateContext)]
#[validate(schema(
  function = "validate_claims",
  skip_on_field_errors = false,
  use_context,
))]
pub struct ValidClaims {
  #[validate(length(equal = 36, message = "无效的用户ID!"))]
  pub sub: String, // 标准字段: 用户ID
  pub exp: i64, // 标准字段: 过期时间
  pub iat: i64, // 签发时间: 签发时间
  // #[validate(length(min = 11, message = "手机号码错误!"))]
  pub phone: String, // 自定义字段: 用户手机号
}

pub fn validate_claims(
  claims: &ValidClaims,
  context: &ValidateContext,
) -> Result<(), ValidationError> {
  let user_id = claims.sub.as_str();
  let user_phone = claims.phone.as_str();
  let exp = claims.exp;
  let iat = claims.iat;

  let error = ValidationError::new("validate_claims");

  match user_id == context.user.user_id.as_str() {
    // 用户id正确
    true => {
      match user_phone == context.user.user_phone.as_str() {
        // 手机号正确
        true => {
          match exp > iat {
            // 过期时间正确
            true => Ok(()),
            // 过期时间错误
            false => Err(error.with_message("该用户登录状态失效.".into())),
          }
        }
        // 手机号错误
        false => Err(ValidationError::new("用户ID与手机号不匹配.".into())),
      }
    }
    // 用户ID错误
    false => Err(error.with_message("该用户未注册".into())),
  }
}

#[derive(Debug, Error)]
pub enum ServerError {}

#[cfg(test)]
mod tests {
  use crate::ValidClaims;
  use sqlx::postgres::PgPoolOptions;
  use std::time::Duration;
  use validator::ValidateArgs;

  #[tokio::test(flavor = "multi_thread")]
  async fn validate_claims() -> anyhow::Result<()> {
    let valid_claims = ValidClaims {
      sub: "".to_string(),
      exp: 0,
      iat: 0,
      phone: "".to_string(),
    };
    let pool = PgPoolOptions::new()
      // .max_connections((num_cpus::get() * 2) + 2)
      .max_connections(2)
      .acquire_timeout(Duration::from_millis(6000))
      .connect("postgres://postgres:postgres@localhost/paotui")
      .await?;

    // let res = valid_claims.validate_with_args(&pool)?;

    println!("hello");
    Ok(())
  }
}
