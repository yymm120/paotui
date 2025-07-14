use crate::utils::db::Db;

#[derive(Clone, Debug)]
pub struct AppState {
  pub db: Db,
}
