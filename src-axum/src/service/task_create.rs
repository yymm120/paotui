use crate::model::app_state::AppState;
use crate::model::auth::{Profile, ResponseForLogin};
use crate::model::table::address::Address;
use crate::model::table::delivery_task::DeliveryTask;
use crate::model::task::{DeliveryTaskCreateData, RequestBodyForTaskCreate, Task};
use crate::query::query_address::create_address_if_not_exist;
use crate::query::delivery_task::create_delivery_task;
use crate::service::error::{Result, ServiceError};
use axum::Json;
use chrono::Local;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, instrument};
use uuid::Uuid;

#[instrument]
pub async fn service(
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  task_body: RequestBodyForTaskCreate,
) -> Result<Json<Task>> {
  // 查询余额并扣款, 余额查询失败错误， 余额不足错误
  pay_for_create_delivery_task(profile.clone(), &task_body);
  // 创建任务， 任务创建失败，要进行重试，3次都创建失败，则退回客户金额。
  let (addr_send, addr_receive, delivery_task) =
    db_for_create_delivery_task(profile.clone(), app_state, &task_body).await?;

  // TODO: 计算距离 distance_current_to_store

  Ok(Json::from(Task {
    id: delivery_task.id,
    time_order: delivery_task.create_time.to_string(),
    time_arrived: delivery_task.arrived_time.map(|t| t.to_string()),
    time_expected_arrived: None,
    time_remaining: None,
    time_pickup: None,
    address_send: task_body.user_send.location,
    address_receive: task_body.user_receive.location,
    distance_current_to_store: 0,
    distance_store_to_customer: 0,
    distance_current_to_customer: 0,
    tag: delivery_task.tag,
    items: delivery_task.items,
    note: delivery_task.note,
    status: delivery_task.status,
    priority: delivery_task.priority,
    estimated_in_come: delivery_task.estimated_income,
    telephone_send: delivery_task.telephone_send,
    username_send: delivery_task.username_send,
    telephone_receive: delivery_task.telephone_receive,
    username_receive: delivery_task.username_receive,
    telephone_delivery: None,
    username_delivery: None,
  }))
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

pub async fn pay_for_create_delivery_task(
  profile: Arc<Mutex<Profile>>,
  task_body: &RequestBodyForTaskCreate,
) -> Result<bool> {
  // Err(ServiceError::PaymentDeductionFailed(""))
  Ok(true)
}

pub async fn db_for_create_delivery_task(
  profile: Arc<Mutex<Profile>>,
  app_state: AppState,
  task_body: &RequestBodyForTaskCreate,
) -> Result<(Address, Address, DeliveryTask)> {
  let db = app_state.db;
  let profile_clone;
  {
    let mut profile_guard = profile.lock().unwrap();
    profile_clone = profile_guard.clone();
  }

  // 从数据库中查询地址是否存在, 然后创建地址
  let address_for_send = create_address_if_not_exist(&db, &task_body.user_send.location).await;
  if address_for_send.is_err() {
    return Err(ServiceError::DBOperationFailed(
      "create start address occur error!".to_string(),
    ));
  }
  let address_for_receive = create_address_if_not_exist(&db, &task_body.user_send.location).await;
  if address_for_receive.is_err() {
    return Err(ServiceError::DBOperationFailed(
      "create end address occur error!".to_string(),
    ));
  }

  let addr_send = address_for_send.unwrap();
  let addr_receive = address_for_receive.unwrap();

  // TODO: 计算预估费用, 默认为2.5

  // 构建TaskCreateData数据
  let data_for_create_task = DeliveryTaskCreateData {
    address_start_id: addr_send.id,
    address_end_id: addr_receive.id,
    tag: None,
    items: None,
    note: None,
    priority: Some("default".to_string()),
    estimated_income: "2.5".to_string(),
    username_send: task_body.user_send.name.clone(),
    username_receive: task_body.user_receive.name.clone(),
    username_delivery: None,
    telephone_send: task_body.user_send.phone_number.clone(),
    telephone_receive: task_body.user_receive.phone_number.clone(),
    telephone_delivery: None,
    user_id_delivery_by: None,
    user_id_created_by: profile_clone.user_id,
  };

  // 创建Delivery Task
  let delivery_task = create_delivery_task(&db, data_for_create_task).await;

  debug!("delivery_task: {:?}", delivery_task);

  match delivery_task {
    Ok(task) => Ok((addr_send, addr_receive, task)),
    Err(e) => Err(ServiceError::DBOperationFailed(format!(
      "create delivery_task occur error! {:?}",
      e
    ))),
  }
}
