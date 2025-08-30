use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Address {
  pub id: i64,

  pub poiname: String,

  pub cityname: String,

  pub poiaddress: String,

  pub latlng_lat: f64,

  pub latlng_lng: f64,

  pub created_at: Date,
  pub updated_at: Date,
}
