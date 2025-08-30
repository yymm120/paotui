// use crate::model::table::address::Address;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBodyForTaskCreate {
  pub shipping_method: String,
  pub user_send: UserSend,
  pub user_receive: UserReceive,
  pub need_save: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBodyForTaskUpdate {
  /// 骑手位置
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  pub poiname: String,
  pub cityname: String,
  pub poiaddress: String,
  pub latlng_lat: f64,
  pub latlng_lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSend {
  pub name: String,
  pub phone_number: String,
  pub floor_room_number: String,
  pub location: Location,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserReceive {
  pub name: String,
  pub phone_number: String,
  pub floor_room_number: String,
  pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
  // id: 年月日+第几单+时分秒
  pub id: String,
  // 跑腿任务的下发时间
  pub time_order: String,
  // 跑腿实际的送达时间
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_arrived: Option<String>,
  // 顾客期望的送达时间
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_expected_arrived: Option<String>,
  // 跑腿的剩余时间
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_remaining: Option<(i32, i32, i32)>, // 天 时 分
  // 跑腿的接单时间
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_pickup: Option<String>,
  // 店铺地址
  pub address_send: Location,
  // // 骑手当前地址
  // #[serde(skip_serializing_if = "Option::is_none")]
  // pub address_current: Option<Location>,
  // 顾客地址
  pub address_receive: Location,
  // 当前地址到店铺的距离
  pub distance_current_to_store: i32,
  // 店铺到顾客的距离
  pub distance_store_to_customer: i32,
  // 当前地址到顾客的距离
  pub distance_current_to_customer: i32,
  // 标记
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
  // 货品清单
  #[serde(skip_serializing_if = "Option::is_none")]
  pub items: Option<String>,
  // 备注
  #[serde(skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
  // 运送状态
  pub status: String,
  // 运送优先级
  pub priority: String,
  // 预计收入
  pub estimated_in_come: String,
  // 发送者电话
  pub telephone_send: String,
  // 商家名称
  pub username_send: String,
  // 顾客电话
  pub telephone_receive: String,
  // 顾客名称
  pub username_receive: String,
  // 顾客电话
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telephone_delivery: Option<String>,
  // 顾客名称
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username_delivery: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeliveryTaskCreateData {
  pub address_start_id: i64,
  pub address_end_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub items: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub note: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<String>,
  pub estimated_income: String,
  pub username_send: String,
  pub username_receive: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username_delivery: Option<String>,
  pub telephone_send: String,
  pub telephone_receive: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub telephone_delivery: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id_delivery_by: Option<String>,
  pub user_id_created_by: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DeliveryTaskUpdateData {
  pub id: Option<String>,
  pub create_time: Option<PrimitiveDateTime>,
  pub arrived_time: Option<PrimitiveDateTime>,
  pub address_start_id: Option<i64>,
  pub address_current_id: Option<i64>,
  pub address_end_id: Option<i64>,
  pub tag: Option<String>,
  pub items: Option<String>,
  pub note: Option<String>,
  pub status: Option<String>,
  pub priority: Option<String>,
  pub estimated_income: Option<String>,
  pub username_send: Option<String>,
  pub username_receive: Option<String>,
  pub username_delivery: Option<String>,
  pub telephone_send: Option<String>,
  pub telephone_receive: Option<String>,
  pub telephone_delivery: Option<String>,
  pub user_id_delivery_by: Option<String>,
  pub user_id_created_by: Option<String>,
}
