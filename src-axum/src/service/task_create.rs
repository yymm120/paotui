use crate::model::app_state::AppState;
use crate::model::auth::Profile;
use crate::model::task::RequestBodyForTaskCreate;
use std::sync::{Arc, Mutex};
use chrono::Local;
use uuid::Uuid;

pub async fn service(
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  task_body: RequestBodyForTaskCreate,
) {
}

pub async fn mock_service(
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  task_body: RequestBodyForTaskCreate,
) {
  // 1. id
  let id = Uuid::new_v4();
  // 2. 跑腿任务的下发时间
  let time_order = Local::now();
  // 3. 跑腿的送达时间
  let time_arrived = time_order.timestamp();
  // 4. 跑腿的剩余时间
  let time_remaining = time_arrived;
  // 5. 跑腿的接单时间
  let time_pickup = 1;
  // 6. 店铺地址
  let address_store = "".to_string();
  // 7. 当前地址
  let address_current = "";
  // 8. 顾客地址
  let address_customer = "";
  // 9. 当前地址到店铺的距离
  let distance_current_to_store = "";
  // 10. 店铺到顾客的距离
  let distance_store_to_customer = "";
  // 11. 当前地址到顾客的距离
  let distance_current_to_customer = "";
  // 12. 标记
  let tag = Some("");
  // 13. 货品清单
  let items = "";
  // 14. 备注
  let note = Some("");
  // 15. 运送状态
  let status = "";
  // 16. 运送优先级
  let priority = "";
  // 17. 预计收入
  let estimated_in_come = "";
  // 18. 商家电话
  let telephone_store = "";
  // 19. 商家名称
  let name_store = "";
  // 20. 顾客电话
  let telephone_customer = "";
  // 21. 顾客名称
  let name_customer = "";
}
