use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Txmap;
#[cfg(mobile)]
use mobile::Txmap;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the txmap APIs.
pub trait TxmapExt<R: Runtime> {
  fn txmap(&self) -> &Txmap<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TxmapExt<R> for T {
  fn txmap(&self) -> &Txmap<R> {
    self.state::<Txmap<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("txmap")
    .invoke_handler(tauri::generate_handler![
      commands::fragment,
      commands::start_tx_map_with_channel,
      commands::add_task,
      commands::clear_task,
      commands::clear_channel,
      commands::watch_location,
      commands::watch_direction,
      commands::clear_watch_location,
      commands::clear_watch_direction,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let txmap = mobile::init(app, api)?;
      #[cfg(desktop)]
      let txmap = desktop::init(app, api)?;
      app.manage(txmap);
      Ok(())
    })
    .build()
}
