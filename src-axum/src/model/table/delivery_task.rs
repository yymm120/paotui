use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{Date, OffsetDateTime, PrimitiveDateTime};

/// 配送任务表
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeliveryTask {
  /// 订单id，也是订单编号，应该有一种固定的编号规则
  pub id: String,

  /// 客户下订单的下单时间，也是创建时间
  pub create_time: PrimitiveDateTime,

  /// 配送抵达时间
  pub arrived_time: Option<PrimitiveDateTime>,

  /// 配送的起点，也是骑手取件的地点
  pub address_start_id: i64,

  /// 配送的终点，必须设置
  pub address_end_id: i64,

  /// 物品标签，可以为空
  pub tag: Option<String>,

  /// 配送的物品，可以为空
  pub items: Option<String>,

  /// 注释
  pub note: Option<String>,

  /// 配送的状态，包括：new / pending / done 三个状态
  pub status: String,

  /// 优先级，表明是否优先配送
  pub priority: String,

  /// 预计收入，指的是骑手的收入
  pub estimated_income: String,

  /// 谁发的
  pub username_send: String,

  /// 谁收的
  pub username_receive: String,

  /// 谁送的，未接单时，为空
  pub username_delivery: Option<String>,

  /// 发送者电话
  pub telephone_send: String,

  /// 接收者电话
  pub telephone_receive: String,

  /// 配送者电话
  pub telephone_delivery: Option<String>,

  /// 配送者id
  pub user_id_delivery_by: Option<String>,

  /// 谁创建的
  pub user_id_created_by: String,
}

// impl Serialize for OffsetDateTime {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     // 格式化为ISO8601字符串
//     serializer.serialize_str(&self.format(&time::format_description::well_known::Iso8601)?)
//   }
// }
