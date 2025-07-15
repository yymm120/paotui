use std::sync::{Arc, Mutex};
use crate::model::auth::Profile;
use crate::model::task::RequestBodyForTaskUpdate;

pub async fn service() {

}
pub async fn mock_service(profile: Arc<Mutex<Profile>>, id: Option<i32>, task_body: RequestBodyForTaskUpdate) {

}