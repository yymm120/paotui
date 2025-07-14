use crate::AppState;
use crate::model::auth::{Profile, RequestBodyForLogin, ResponseForLogin, UserType};
use crate::model::table::user_table::UserTable;
use crate::query::query_user::{create_user, query_user_by_phone};
use crate::service::{
  common::verify_login_code,
  error::{Result, ServiceError},
};
use crate::utils::db::Db;
use crate::utils::jwt::{generate_claims, generate_token};
use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::Response;
use axum::{Extension, Json, http};
use sqlx::Error;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::instrument;
use uuid::Uuid;

#[instrument]
pub async fn login(
  header_map: HeaderMap<HeaderValue>,
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  login_body: RequestBodyForLogin,
) -> Result<Json<ResponseForLogin>> {
  let code = login_body.code;
  let db = app_state.db;
  let profile_clone;
  let token;
  let user_type;
  {
    let mut profile_guard = profile.lock().unwrap();
    profile_clone = profile_guard.clone();
    token = profile_guard.token.clone();
    user_type = profile_guard.user_type.clone();
  }

  // 检查用户类型
  match profile_clone.user_type {
    // 无需继续登录, 直接返回 TODO: 已登录用户也许不应该获取登录验证码, 在code_handler中就应该阻止
    UserType::VerifiedUser => Ok(Json::from(ResponseForLogin { status: false })),
    // 检查短信验证码, 创建用户
    UserType::UnsignedUpUser | UserType::UnSignedInUser => {
      let user_phone = login_body.user_phone;
      let exist_user = query_user_by_phone(&db, user_phone.clone()).await;
      match exist_user {
        // 果然存在, 这是未登录用户, 现在需要登录
        Some(user_item) => {
          let status = verify_login_code(user_phone.as_str(), code.as_str())
            .await
            .unwrap_or(false);
          match status {
            true => {
              let claims = generate_claims(UserType::VerifiedUser, Some(user_item));
              let token = generate_token(UserType::VerifiedUser, claims)?;
              {
                let mut profile = profile.lock().unwrap();
                profile.token = token.clone(); // TODO 新token
              }
              Ok(Json::from(ResponseForLogin { status: true }))
            }
            false => Err(ServiceError::UnauthorizedError("验证码错误!")),
          }
        }
        None => {
          let status = verify_login_code(user_phone.as_str(), code.as_str())
            .await
            .unwrap_or(false);
          match status {
            true => {
              let new_user_item =
                create_user(&db, profile_clone.user_id.clone(), user_phone.clone()).await?;
              let claims = generate_claims(UserType::VerifiedUser, Some(new_user_item));
              let token = generate_token(UserType::VerifiedUser, claims)?;
              {
                let mut profile = profile.lock().unwrap();
                profile.token = token.clone(); // TODO 新token
              }
              Ok(Json::from(ResponseForLogin { status: true }))
            }
            false => Err(ServiceError::UnauthorizedError("验证码错误!")),
          }
        }
      }
    } // // 检查短信验证码, 修改exp
      // UserType::UnSignedInUser => {
      //   // 未登录是根据token来检查的, 最终要回到用户输入的手机号来检查, 所以这里需要再次检查
      //   let user_item_op = query_user_by_phone(&db, login_body.user_phone).await;
      //
      //   // 如果手机号在数据库中能找到, 才能说明这是未登录用户
      //   match user_item_op {
      //     Some(user_item) => {
      //       // 找到了该用户, 的确是未登录用户
      //       // 真正的user_id, 也需要以数据库中的user_id为准, 所以middleware中生成的claims需要更新
      //       let claims = generate_claims(UserType::UnSignedInUser, Some(user_item)).expect("只要user_item存在, 这里不可能出错.");
      //       // 然后修改profile中的user_id和user_phone
      //       profile.user_id = claims.sub;
      //       profile.user_phone = claims.phone;
      //       profile.user_type = UserType::UnSignedInUser;
      //       let token = generate_token(UserType::UnSignedInUser, &Some(claims.clone()));
      //       profile.token = "a".to_string(); // 过期时间需要清空, 才能表示该用户还没有登录
      //
      //       match login_unsigned_in_user(user_item, token, &db, code.as_str()).await {
      //         Ok(res) => {
      //           profile.user_type = UserType::VerifiedUser;
      //           Ok(res)
      //         }
      //         Err(e) => {
      //           profile.user_type = UserType::UnSignedInUser;
      //           Err(e)
      //         }
      //       }
      //     }
      //     None => {
      //       match login_unsigned_up_user(user_item, token, &db, code.as_str()).await {
      //         Ok(res) => {
      //           profile.user_type = UserType::VerifiedUser;
      //           Ok(res)
      //         }
      //         Err(e) => {
      //           profile.user_type = UserType::UnSignedInUser;
      //           Err(e)
      //         }
      //       }
      //     }
      //   }

      // 然后开始校验登录验证码
      // let status = verify_login_code(code.as_str());
      // generate_jwt()

      // let response = login_unsigned_user(user_item, profile, &db, code.as_str());
      // if (code == "1234") {
      //   // 验证码正确
      //   Ok(Response::builder()
      //     .status(StatusCode::OK)
      //     .header(AUTHORIZATION, token)
      //     .header("Content-Type", "application/json")
      //     .body::<ResponseForLogin>(ResponseForLogin { status: false })?)
      // } else {
      //   Ok(Response::builder()
      //     .status(StatusCode::OK)
      //     .header(AUTHORIZATION, token)
      //     .header("Content-Type", "application/json")
      //     .body::<ResponseForLogin>(ResponseForLogin { status: false })?)
      // }
      // }
  }
}

async fn login_unsigned_in_user(
  user_item: UserTable,
  token: String,
  db: &Db,
  code: &str,
) -> Result<Json<ResponseForLogin>> {
  // 校验登录验证码
  let status = verify_login_code(user_item.user_phone.as_str(), code).await;
  match status.unwrap_or(false) {
    // 验证码正确
    true => {
      let claims = generate_claims(UserType::VerifiedUser, Some(user_item));
      let token = generate_token(UserType::VerifiedUser, claims)?;
      Ok(Json::from(ResponseForLogin { status: false }))
    }
    // 验证码错误
    false => {
      let claims = generate_claims(UserType::UnSignedInUser, Some(user_item));
      let token = generate_token(UserType::UnSignedInUser, claims)?;
      Ok(Json::from(ResponseForLogin { status: false }))
    }
  }
}

// async fn login_verified_user(token: String) -> Result<Json<ResponseForLogin>> {
//   Ok(Json::from(ResponseForLogin { status: false }))
//   // Ok(Response::builder()
//   //   .status(StatusCode::OK)
//   //   .header(AUTHORIZATION, token)
//   //   .header("Content-Type", "application/json")
//   //   .body::<ResponseForLogin>(ResponseForLogin { status: true })?)
// }

async fn login_unsigned_up_user(
  user_item: UserTable,
  token: String,
  db: &Db,
  code: &str,
) -> Result<Json<ResponseForLogin>> {
  // let user_id = profile.user_id.clone();
  // let user_phone = profile.user_phone.clone();
  //
  // let claims_a = match user_item_res {
  //   Ok(user_item) => {
  //     generate_claims(UserType::VerifiedUser)
  //
  //   }
  //   Err(e) => {}
  // };
  //
  // let claims_op = generate_claims(user_item);
  // let token = generate_token(profile.user_type.clone(), &claims_op);
  //
  // let token = profile.clone().token;
  Ok(Json::from(ResponseForLogin { status: false }))
}
