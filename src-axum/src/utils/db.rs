use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres, Transaction};
use std::time::Duration;
use tracing::info;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool(db_con_url: &str) -> Result<Db, sqlx::Error> {
  info!("--> into new db pool");
  info!("--> {:?}", db_con_url);
  PgPoolOptions::new()
    // .max_connections((num_cpus::get() * 2) + 2)
    .max_connections(2)
    .acquire_timeout(Duration::from_millis(6000))
    .connect(db_con_url)
    .await
}
// 在SQLx中，&str类型被视为非预处理查询，而Query或QueryAs结构体则被视为预处理查询。

// pub fn query_user_by_id
