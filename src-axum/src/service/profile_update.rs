use std::str::FromStr;
use std::sync::{Arc, Mutex};
use axum::Json;
use tracing::debug;
use crate::model::app_state::AppState;
use crate::model::auth::Profile;
use crate::model::pg::delivery_app_user::{DeliveryAppUser, UpdateDeliveryAppUser};
use crate::model::pg::delivery_person::{ClientProfile, DeliveryPerson, ProfileStatus, UpdateDeliveryPerson, UpdateStatusRequestBody};
use crate::model::task::{RequestBodyForTaskCreate, Task};
use crate::model::pg::delivery_app_user::update_delivery_app_user;
use crate::model::pg::delivery_person::{update_delivery_person};
use crate::model::pg::task_view::query_task_view_by_id;
use crate::service::error::{Result, ServiceError};
use crate::utils::db::Db;

pub async fn service(
  profile_arc: Arc<Mutex<Profile>>,
  app_state: AppState,
  UpdateStatusRequestBody {status}: UpdateStatusRequestBody
) -> Result<Json<ClientProfile>> {
  let user_id = profile_arc.lock().unwrap().user_id.clone();

  let db = app_state.db;
  let delivery_user = update_delivery_user_working_status(&db, i64::from_str(&user_id).unwrap(), status).await?;
  let status = ProfileStatus::from_str(&delivery_user.status).unwrap();
  let mut profile_clone;
  {
    let mut profile = profile_arc.lock().unwrap();
    profile.status = status.clone();
    profile_clone = profile.clone();
  }

  Ok(Json(ClientProfile {
    user_type: profile_clone.user_type,
    user_name: profile_clone.user_name,
    user_phone: profile_clone.user_phone,
    status,
  }))
}

async fn update_delivery_user_working_status(db: &Db, id: i64, status: String) -> Result<DeliveryAppUser> {
  let mut delivery_person = UpdateDeliveryPerson::default();
  // delivery_person.delivery_person_status = Some(status);
  let delivery_user = update_delivery_app_user(db, UpdateDeliveryAppUser::change_status(id, status)).await?;
  Ok(delivery_user)
}
