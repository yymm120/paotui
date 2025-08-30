use crate::model::auth::Profile;
use std::sync::{Arc, Mutex};

pub async fn service() {}
pub async fn mock_service(profile: Arc<Mutex<Profile>>, id: Option<i32>) {}
