use crate::model::app_state::AppState;
use crate::model::auth::Profile;
use crate::model::pg::delivery_task::DeliveryTask;
use crate::model::task::{Location, Task};
use crate::model::view::task_view::TaskView;
use crate::model::pg::address::query_address_by_id;
use crate::model::pg::delivery_task::{
  query_delivery_task_by_id, query_delivery_task_list_by_status,
};
use crate::model::pg::task_view::{query_task_view_by_status, query_task_view_by_owner};
use crate::service::error::Result;
use crate::utils::db::Db;
use axum::Json;
use std::sync::{Arc, Mutex};
use anyhow::Context;

pub async fn service(
  profile: Arc<Mutex<Profile>>,
  id: Option<String>,
  status: Option<String>,
  owned: Option<bool>,
  app_state: AppState,
) -> Result<Json<Vec<Task>>> {
  let db = app_state.db;
  let profile_clone;
  {
    let mut profile_guard = profile.lock().unwrap();
    profile_clone = profile_guard.clone();
  }
  match id {
    None => list_all_task(&db, profile_clone, status, owned).await,
    Some(id) => list_one_task(),
  }
}

async fn list_all_task(db: &Db, profile: Profile, status: Option<String>, owned: Option<bool>) -> Result<Json<Vec<Task>>> {
  let mut delivery_task_list = query_task_view_by_status(db, status.unwrap_or("idle".to_string())).await?;
  let user_id = profile.user_id.parse::<i32>().context("cannot parse user id.")?;
  let mut delivery_task_list = query_task_view_by_owner(db, user_id).await?;
  let task_list = delivery_task_list
    .into_iter()
    .map(|delivery_task| normalize_for_task(delivery_task))
    .collect::<Vec<Task>>();

  // delivery_task_list

  Ok(Json(task_list))
}



fn normalize_for_task(task_view: TaskView) -> Task {
  let task = Task {
    id: task_view.id.unwrap(),
    time_order: task_view.create_time.unwrap().to_string(),
    time_arrived: task_view.arrived_time.map(|t| t.to_string()),
    time_expected_arrived: None,
    time_remaining: None,
    time_pickup: None,
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
  };
  task
}

fn list_one_task() -> Result<Json<Vec<Task>>> {
  todo!()
}

pub async fn mock_service(profile: Arc<Mutex<Profile>>, app_state: AppState, id: Option<i32>) {}
