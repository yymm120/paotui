//! =====================
//!
//! Meta Information:
//! - name: 'paotui.public.delivery_task'
//! - sync: true
//! - Enum: [Col]
//! - Struct: [DeliveryTask], [DeliveryTaskForCreate], [DeliveryTaskForUpdate]
//! - methods: [insert], [update], [delete], [retrieve]
//! - oid: #40238#
//!
//! Note: Please do not delete comments, the code generation tool relies on comments.
//!
//! Examples:
//! ```rust
//! fn main () {
//!   let transaction = pool.begin();
//!   let data = DeliveryForCreate::default();
//!   model::insert(&mut *transaction, &data);
//!   transaction.commit();
//! }
//! ```
//!
//! =====================
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, PgPool, Postgres};
#[derive(Debug)]
enum Col {
    Id(i64),
    Latitude(f64),
    Longitude(f64),
    Altitude(f64),
    Accuracy(f64),
    CreatedAt(chrono::NaiveDate),
    UpdatedAt(chrono::NaiveDate),
    DeletedAt(chrono::NaiveDate),
}
#[derive(Debug, FromRow, Serialize, Deserialize, Clone, Default)]
pub struct Address {
    id: i64,
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
    accuracy: Option<f64>,
    created_at: chrono::NaiveDate,
    updated_at: chrono::NaiveDate,
    deleted_at: Option<chrono::NaiveDate>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddressForCreate {
    id: Option<i64>,
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
    accuracy: Option<f64>,
    created_at: Option<chrono::NaiveDate>,
    updated_at: Option<chrono::NaiveDate>,
    deleted_at: Option<chrono::NaiveDate>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddressForUpdate {
    id: i64,
    latitude: Option<f64>,
    longitude: Option<f64>,
    altitude: Option<f64>,
    accuracy: Option<f64>,
    created_at: Option<chrono::NaiveDate>,
    updated_at: Option<chrono::NaiveDate>,
    deleted_at: Option<chrono::NaiveDate>,
}
pub async fn insert<'a, E>(
    executor: E,
    data: &AddressForCreate,
) -> Result<Address, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let mut cols: Vec<Col> = Vec::new();
    let mut column_names = Vec::new();
    let mut placeholders = Vec::new();
    match &data.id {
        None => {}
        Some(val) => {
            cols.push(Col::Id(val.clone()));
            column_names.push(stringify!(id));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    cols.push(Col::Latitude(data.latitude.clone()));
    column_names.push(stringify!(latitude));
    placeholders.push(format!("${}", column_names.len()));
    cols.push(Col::Longitude(data.longitude.clone()));
    column_names.push(stringify!(longitude));
    placeholders.push(format!("${}", column_names.len()));
    match &data.altitude {
        None => {}
        Some(val) => {
            cols.push(Col::Altitude(val.clone()));
            column_names.push(stringify!(altitude));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.accuracy {
        None => {}
        Some(val) => {
            cols.push(Col::Accuracy(val.clone()));
            column_names.push(stringify!(accuracy));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.created_at {
        None => {}
        Some(val) => {
            cols.push(Col::CreatedAt(val.clone()));
            column_names.push(stringify!(created_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.updated_at {
        None => {}
        Some(val) => {
            cols.push(Col::UpdatedAt(val.clone()));
            column_names.push(stringify!(updated_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    };
    match &data.deleted_at {
        None => {}
        Some(val) => {
            cols.push(Col::DeletedAt(val.clone()));
            column_names.push(stringify!(deleted_at));
            placeholders.push(format!("${}", column_names.len()));
        }
    }
    let sql = format!(
        r#"INSERT INTO {} ( {} ) VALUES ( {} ) RETURNING {} "#, "paotui.public.address",
        column_names.join(", "), placeholders.join(", "), stringify!(id, latitude,
        longitude, altitude, accuracy, created_at, updated_at, deleted_at)
    );
    let mut query = sqlx::query_as(&sql);
    for col in cols {
        match col {
            Col::Id(val) => {
                query = query.bind(val);
            }
            Col::Latitude(val) => {
                query = query.bind(val);
            }
            Col::Longitude(val) => {
                query = query.bind(val);
            }
            Col::Altitude(val) => {
                query = query.bind(val);
            }
            Col::Accuracy(val) => {
                query = query.bind(val);
            }
            Col::CreatedAt(val) => {
                query = query.bind(val);
            }
            Col::UpdatedAt(val) => {
                query = query.bind(val);
            }
            Col::DeletedAt(val) => {
                query = query.bind(val);
            }
            _ => {}
        }
    }
    query.fetch_one(executor).await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &AddressForUpdate,
) -> Result<Address, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Address,
        "UPDATE paotui.public.address
         SET
           latitude = COALESCE($1, latitude),
           longitude = COALESCE($2, longitude),
           altitude = COALESCE($3, altitude),
           accuracy = COALESCE($4, accuracy),
           created_at = COALESCE($5, created_at),
           updated_at = COALESCE($6, updated_at),
           deleted_at = COALESCE($7, deleted_at)
         WHERE id = $8
         RETURNING id,latitude,longitude,altitude,accuracy,created_at,updated_at,deleted_at
         ",
        data.latitude, data.longitude, data.altitude, data.accuracy, data.created_at,
        data.updated_at, data.deleted_at, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn update_safe<'a, E>(
    executor: E,
    data: &AddressForUpdate,
) -> Result<Address, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(
        Address,
        "UPDATE paotui.public.address
         SET
           latitude = COALESCE($1, latitude),
           longitude = COALESCE($2, longitude),
           altitude = COALESCE($3, altitude),
           accuracy = COALESCE($4, accuracy),
           created_at = COALESCE($5, created_at),
           updated_at = COALESCE($6, updated_at),
           deleted_at = COALESCE($7, deleted_at)
         WHERE id = $8
         RETURNING id,latitude,longitude,altitude,accuracy,created_at,updated_at,deleted_at
         ",
        data.latitude, data.longitude, data.altitude, data.accuracy, data.created_at,
        data.updated_at, data.deleted_at, data.id
    )
        .fetch_one(executor)
        .await
}
pub async fn delete<'a, E>(executor: E, id: &i64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM paotui.public.address WHERE id = $1", id)
        .execute(executor)
        .await?;
    Ok(())
}
pub async fn query_by_primary_key<'a, E>(
    executor: E,
    id: i64,
) -> Result<Address, sqlx::Error>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query_as!(Address, "SELECT * FROM paotui.public.address WHERE id = $1", id)
        .fetch_one(executor)
        .await
}



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
