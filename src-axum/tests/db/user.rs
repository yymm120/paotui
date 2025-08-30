#![allow(unused)]

use crate::db::common::{User, conn_mutable, mapping_instance_case1, mapping_instance_case2, pool};
use futures::TryStreamExt;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Connection, Error, Executor, PgConnection, Pool, Postgres, Row};
use std::sync::OnceLock;
use uuid::Uuid;

/// Sqlx 有两种查询
/// - 预处理（参数化）查询：查询会被缓存、采用二进制通信模式（降低带宽占用且解码更快），通过参数化机制防范SQL注入。
/// - 非预处理（简单）查询：仅适用于预处理语句无法实现的场景，例如执行各类数据库命令（如PRAGMA、SET或BEGIN等）。
///
/// SQLx框架完整支持这两种查询类型的全部操作。
/// - `&str`类型字符串会被视为非预处理查询，
/// - 而`Query`或`QueryAs`结构体则被识别为预处理查询。
///
/// Sqlx 有两种查询终止器（`execute`和`fetch_xxx`）
/// - `execute`会返回受影响的行数（如果有的话），并**丢弃所有结果**。
/// - `fetch`,`fetch_one`,`fetch_optional` 和 `fetch_all` 等方法用于**接收结果**。
///
///
/// 通过 `sqlx::query` 返回的 `Query` 类型会从数据库中返回一个 `Row<'conn>` 类型的行。可以通过序号或名称使用 `row.get()` 方法来访问列的值。
/// 由于 `Row` 对连接持有不可变借用，因此同一时间只能存在一个 `Row` 实例。而 `fetch` 查询终结器会返回一个类似流的类型，用于遍历结果集中的每一行。

#[tokio::test(flavor = "multi_thread")]
async fn query_pre_builder() -> anyhow::Result<()> {
  let mut pool = pool().await;

  // case1: 预处理查询 (参数化查询)
  let res = pool
    .execute(sqlx::query("SELECT * FROM public.user"))
    .await?;
  assert_eq!(16, res.rows_affected());

  // case2: 非预处理查询 (简单查询)
  let res = pool.execute("SELECT * FROM user_table").await?;
  assert_eq!(16, res.rows_affected());

  // Note:
  // 一般不使用 pool.execute(query) 这种低级调用方式
  // 推荐使用 sqlx::query().execute(&pool) 或 sqlx::query().execute(&mut pool) 这种高级调用方式
  // 为此, sqlx::query 内置了 execute 和 fetch_xxx 方法
  let mut conn = conn_mutable().await?;

  // case3: fetch 方法返回流式类型, 需要通过遍历获取每一行的数据
  let query = sqlx::query("SELECT * FROM public.user");
  let mut result_stream = query.fetch(&mut conn);
  /* try_next 需要依赖项 use futures::TryStreamExt; */
  /* try_get  需要依赖项 use sqlx::Row; */
  while let Some(row) = result_stream.try_next().await? {
    // let user_id: Uuid = row.try_get("user_id")?;
    // println!("{:?}", user_id.to_string());
  }

  Ok(())
}

// 通常不太希望通过遍历的方式获取数据, 而是直接映射成一个类型实例
// `query!`宏和`query_as!`宏可以直接映射成实例
/// 输入（或绑定）参数必须一次性全部提供（且会在编译时验证其数量和类型是否正确）。
/// • 输出类型为一个匿名记录。以上述示例而言，该类型类似于：

/// 映射为一个实例
/// 运行: `cargo watch -c -q -x "test query_mapping_instance -- --nocapture"`
#[tokio::test(flavor = "multi_thread")]
async fn query_mapping_instance() -> anyhow::Result<()> {
  let user = mapping_instance_case1().await?;
  assert_eq!(
    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
    user.user_id.to_string()
  );

  let user = mapping_instance_case2().await?;
  assert_eq!(
    "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
    user.user_id.to_string()
  );

  Ok(())
}

/// 我们可以使用宏 sqlx::query！ 在编译时对 SQL 语句进行语法和语义验证，并将其输出为一个匿名记录类型，其中每个 SQL 列都对应一个 Rust 字段
/// 运行: `cargo watch -c -q -x "test test3_query_macro_case1 -- --nocapture"`
#[tokio::test(flavor = "multi_thread")]
async fn test3_query_macro_case1() -> anyhow::Result<()> {
  let pool = pool().await;
  let users = sqlx::query!("SELECT * FROM public.user")
    .fetch_all(pool) // -> Vec<{ country: String, count: i64 }>
    .await?
    .into_iter()
    .map(|record| User {
      user_id: record.user_id,
      username: "".to_string(),
      password: "".to_string(),
      telephone: "".to_string(),
      created_at: record.created_at,
      updated_at: record.updated_at,
      deleted_at: None,
    })
    .collect::<Vec<User>>();
  Ok(())
}

/// 运行: `cargo watch -c -q -x "test test4_query_macro_case2 -- --nocapture"`
#[tokio::test(flavor = "multi_thread")]
async fn test4_query_macro_case2() -> anyhow::Result<()> {
  let pool = pool().await;
  let users = sqlx::query_as!(User, "SELECT * FROM public.user")
    .fetch_all(pool)
    .await?;
  let res = sqlx::query_as!(
    User,
    "INSERT INTO public.user (telephone) VALUES ($1) RETURNING *",
    "123"
  )
  .fetch_optional(pool)
  .await?;
  Ok(())
}
