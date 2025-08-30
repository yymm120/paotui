use crate::model::app_state::AppState;
use crate::model::auth::Profile;
use crate::model::table::delivery_task::DeliveryTask;
use crate::model::task::{DeliveryTaskUpdateData, Location, RequestBodyForTaskUpdate, Task};
use crate::model::view::task_view::TaskView;
use crate::query::query_address::{query_address_by_id, query_address_by_location};
use crate::query::delivery_task::update_delivery_task_non_null;
use crate::query::query_task_view::{query_task_view_by_id, query_task_view_by_status};
use crate::service::error::{Result, ServiceError};
use crate::utils::db::Db;
use crate::utils::time::beijing_time;
use axum::Json;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use tracing::debug;

pub async fn service(
  profile_arc: Arc<Mutex<Profile>>,
  id: String,
  status: String,
  app_state: AppState,
) -> Result<Json<Task>> {
  let db = app_state.db;
  // 检查骑手接单装状态
  let profile = check_working_status_for_delivery_person(profile_arc);
  // 修改配送任务状态
  update_status_for_delivery_task(&db, profile, id.clone(), status).await?;
  let task_view = query_task_view_by_id(&db, id).await?;
  Ok(Json(normalize_task(task_view)))
}

fn normalize_task(task_view: TaskView) -> Task {
  Task {
    id: task_view.id.unwrap(),
    time_order: task_view.create_time.unwrap().to_string(),
    time_arrived: task_view.arrived_time.map(|t| t.to_string()),
    time_expected_arrived: None,
    time_remaining: None,
    time_pickup: Some(beijing_time().to_string()),
    address_send: Location {
      id: None,
      poiname: task_view.start_poiname.unwrap(),
      cityname: task_view.start_cityname.unwrap(),
      poiaddress: task_view.start_poiaddress.unwrap(),
      latlng_lat: task_view.start_lat.unwrap(),
      latlng_lng: task_view.start_lng.unwrap(),
    },
    address_receive: Location {
      id: None,
      poiname: task_view.end_poiname.unwrap(),
      cityname: task_view.end_cityname.unwrap(),
      poiaddress: task_view.end_poiaddress.unwrap(),
      latlng_lat: task_view.end_lat.unwrap(),
      latlng_lng: task_view.end_lng.unwrap(),
    },
    distance_current_to_store: 0,
    distance_store_to_customer: 0,
    distance_current_to_customer: 0,
    tag: task_view.tag,
    items: task_view.items,
    note: task_view.note,
    status: task_view.status.unwrap(),
    priority: task_view.priority.unwrap(),
    estimated_in_come: task_view.estimated_income.unwrap(),
    telephone_send: task_view.telephone_send.unwrap(),
    username_send: task_view.username_send.unwrap(),
    telephone_receive: task_view.telephone_receive.unwrap(),
    username_receive: task_view.username_receive.unwrap(),
    telephone_delivery: task_view.telephone_delivery,
    username_delivery: task_view.username_delivery,
  }
}
fn check_working_status_for_delivery_person(profile: Arc<Mutex<Profile>>) -> Profile {
  let locked_profile = profile.lock().unwrap(); // 获取 MutexGuard
  locked_profile.clone() // 需要 Profile 实现 Clone trait
}

async fn update_status_for_delivery_task(
  db: &Db,
  profile: Profile,
  task_id: String,
  status: String,
) -> Result<DeliveryTask> {

  let mut u = DeliveryTaskUpdateData::default();
  u.id = Some(task_id);
  u.status = Some(status);
  u.user_id_delivery_by = Some(profile.user_id.to_string());
  u.telephone_delivery = Some(profile.user_phone.to_string());
  u.username_delivery = Some(profile.user_name.to_string());
  let delivery_task = update_delivery_task_non_null(db, u).await?;

  Ok(delivery_task)
}

// async fn update_delivery_task_for_access_order(db: &Db, update_data: RequestBodyForTaskUpdate) {
//   let mut u = DeliveryTaskUpdateData::default();
//   u.user_id_delivery_by = Some("".to_string());
//   u.telephone_delivery = Some("".to_string());
//   u.username_delivery = Some("".to_string());
//   let res = update_delivery_task_non_null(db, u).await;
// }

pub async fn mock_service(
  profile: Arc<Mutex<Profile>>,
  id: Option<i32>,
  app_state: AppState,
  task_body: RequestBodyForTaskUpdate,
) {
}
