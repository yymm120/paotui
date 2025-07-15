use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBodyForTaskCreate {
  
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBodyForTaskUpdate {

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
  // 1. id: 年月日+第几单+时分秒
  pub id: String,
  // 2. 跑腿任务的下发时间
  pub time_order: i64,
  // 3. 跑腿的送达时间
  pub time_arrived: i64,
  // 4. 跑腿的剩余时间
  pub time_remaining: (i32, i32, i32), // 天 时 分
  // 5. 跑腿的接单时间
  pub time_pickup: i64,
  // 6. 店铺地址
  pub address_store: String,
  // 7. 当前地址
  pub address_current: String,
  // 8. 顾客地址
  pub address_customer: String,
  // 9. 当前地址到店铺的距离
  pub distance_current_to_store: i32,
  // 10. 店铺到顾客的距离
  pub distance_store_to_customer: i32,
  // 11. 当前地址到顾客的距离
  pub distance_current_to_customer: i32,
  // 12. 标记
  pub tag: Option<String>,
  // 13. 货品清单
  pub items: String,
  // 14. 备注
  pub note: Option<String>,
  // 15. 运送状态
  pub status: String,
  // 16. 运送优先级
  pub priority: String,
  // 17. 预计收入
  pub estimated_in_come: String,
  // 18. 商家电话
  pub telephone_store: String,
  // 19. 商家名称
  pub name_store: String,
  // 20. 顾客电话
  pub telephone_customer: String,
  // 21. 顾客名称
  pub name_customer: String,
}

