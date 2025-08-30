// INSERT INTO address_table (poiname, cityname, poiaddress, latlng_lat, latlng_lng)
// VALUES ('默认地址', '默认城市', '默认详细地址', 0.0, 0.0)
// ON CONFLICT (id) DO NOTHING;

use crate::model::table::address::{Address};
use crate::model::table::user::User;
use crate::model::task::Location;
use crate::query::error::{QueryError, Result};
use crate::utils::db::Db;
use crate::utils::time::beijing_time;
use time::OffsetDateTime;
use tracing::{debug, instrument};

#[instrument]
pub async fn query_address_by_location(db: &Db, location: &Location) -> Result<Address> {
  let address_item = sqlx::query_as!(
    Address,
    "SELECT * FROM public.address WHERE poiname = $1 AND cityname = $2 AND poiaddress = $3 AND latlng_lat = $4 AND latlng_lng = $5",
    location.poiname,
    location.cityname,
    location.poiaddress,
    location.latlng_lat,
    location.latlng_lng,
  )
    .fetch_one(db)
    .await?;

  Ok(address_item)
}

pub async fn query_address_by_id(db: &Db, id: &i64) -> Result<Address> {
  if id < &1000_i64 {
    Err(QueryError::Error("该地址不存在".to_string()))
  } else {
    let address_item = sqlx::query_as!(
      Address,
      "SELECT * FROM public.address WHERE id = $1",
      id
    );
    Err(QueryError::Error("query_address_by_id".to_string()))
  }
}

#[instrument]
pub async fn create_address(db: &Db, location: &Location) -> Result<Address> {
  // 开启事务
  let mut transaction = db.begin().await?;

  // 获取当前时间（UTC）
  let created_at = beijing_time().date();

  let address = sqlx::query_as!(
    Address,
    r#"
        INSERT INTO public.address (
            /* 1 */ poiname,
            /* 2 */ cityname,
            /* 3 */ poiaddress,
            /* 4 */ latlng_lat,
            /* 5 */ latlng_lng,
            /* 6 */ created_at,
            /* 7 */ updated_at
        ) VALUES ($1,$2,$3,$4,$5,$6,$7)
         RETURNING *
        "#,
    /* 1 */ location.poiname,
    /* 2 */ location.cityname,
    /* 3 */ location.poiaddress,
    /* 4 */ location.latlng_lat, // 经度
    /* 5 */ location.latlng_lng, // 纬度
    /* 6 */ created_at, // updated_at
    /* 7 */ created_at, // updated_at
  )
  .fetch_one(&mut *transaction)
  .await?;

  transaction.commit().await?;
  Ok(address)
}

#[instrument]
pub async fn create_address_if_not_exist(db: &Db, location: &Location) -> Result<Address> {
  if location.id.is_some() {
    match query_address_by_id(db, &location.id.unwrap()).await {
      Ok(addr) => Ok(addr),
      Err(e) => {
        debug!("创建新的address");
        Ok(create_address(db, location).await?)
      }
    }
  } else {
    let address = query_address_by_location(&db, location).await;
    match address {
      Ok(addr) => Ok(addr),
      Err(r) => {
        debug!("创建新的address");
        Ok(create_address(db, location).await?)
      }
    }
  }
}
