use tauri::{command, AppHandle, Runtime};
use tauri::ipc::Channel;
use crate::models::*;
use crate::Result;
use crate::TxmapExt;

#[command]
pub(crate) async fn fragment<R: Runtime>(
  app: AppHandle<R>,
  payload: FragmentRequest,
) -> Result<FragmentResponse> {
  app.txmap().fragment(payload)
}

#[command]
pub(crate) async fn start_tx_map_with_channel<R: Runtime>(
  app: AppHandle<R>,
  options: TxMapOptions,
  channel: Channel,
) -> Result<TxMapResponse> {
  app.txmap().start_tx_map_with_channel_inner(options, channel)
}

#[command]
pub(crate) async fn add_task<R: Runtime>(app: AppHandle<R>, payload: AddTaskPayload) -> Result<TxMapResponse> {
  app.txmap().add_task(payload)
}

#[command]
pub(crate) async fn clear_task<R: Runtime>(app: AppHandle<R>, id: String) -> Result<TxMapResponse> {
  app.txmap().clear_task(id)
}

#[command]
pub(crate) async fn clear_channel<R: Runtime>(app: AppHandle<R>, id: u32) -> Result<TxMapResponse> {
  app.txmap().clear_channel(id)
}

#[command]
pub(crate) async fn watch_location<R: Runtime>(app: AppHandle<R>, options: WatchOptions, channel: Channel) -> Result<()> {
  app.txmap().watch_location_inner(options, channel)
}

#[command]
pub(crate) async fn watch_direction<R: Runtime>(app: AppHandle<R>, options: WatchOptions, channel: Channel) -> Result<()> {
  app.txmap().watch_location_inner(options, channel)
}

#[command]
pub(crate) async fn clear_watch_location<R: Runtime>(app: AppHandle<R>, id: u32) -> Result<()> {
  app.txmap().clear_watch_location(id)
}

#[command]
pub(crate) async fn clear_watch_direction<R: Runtime>(app: AppHandle<R>, id: u32) -> Result<()> {
  app.txmap().clear_watch_direction(id)
}