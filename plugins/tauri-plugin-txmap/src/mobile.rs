use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};
use tauri::ipc::{Channel, InvokeResponseBody};
use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_txmap);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Txmap<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.txmap", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_txmap)?;
  Ok(Txmap(handle))
}

/// Access to the txmap APIs.
pub struct Txmap<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Txmap<R> {
  pub fn fragment(&self, payload: FragmentRequest) -> crate::Result<FragmentResponse> {
    self
      .0
      .run_mobile_plugin("fragment", payload)
      .map_err(Into::into)
  }

  pub fn start_tx_map_with_channel<F: Fn(WatchEvent) + Send + Sync + 'static>(&self, options: TxMapOptions, callback: F) -> crate::Result<u32> {
    let channel = Channel::new(move |event| {
      let payload = match event {
        InvokeResponseBody::Json(payload) => serde_json::from_str::<WatchEvent>(&payload)
          .unwrap_or_else(|error| {
            WatchEvent::Error(format!(
              "Couldn't deserialize watch event payload: `{error}`"
            ))
          }),
        _ => WatchEvent::Error("Unexpected watch event payload.".to_string()),
      };

      callback(payload);

      Ok(())
    });
    let id = channel.id();

    self.start_tx_map_with_channel_inner(options, channel)?;
    Ok(id)
  }

  pub(crate) fn start_tx_map_with_channel_inner(
    &self,
    _options: TxMapOptions,
    _callback_channel: Channel,
  ) -> crate::Result<TxMapResponse> {
    self
      .0
      .run_mobile_plugin("startTxMapWithChannel", ChannelPayload { options: _options, channel: _callback_channel })
      .map_err(Into::into)
  }


  pub fn watch_location<F: Fn(WatchEvent) + Send + Sync + 'static>(&self, options: WatchOptions, callback: F) -> crate::Result<u32> {
    let channel = Channel::new(move |event| {
      let payload = match event {
        InvokeResponseBody::Json(payload) => serde_json::from_str::<WatchEvent>(&payload)
          .unwrap_or_else(|error| {
            WatchEvent::Error(format!(
              "Couldn't deserialize watch event payload: `{error}`"
            ))
          }),
        _ => WatchEvent::Error("Unexpected watch event payload.".to_string()),
      };

      callback(payload);

      Ok(())
    });
    let id = channel.id();

    self.watch_location_inner(options, channel)?;
    Ok(id)
  }

  pub(crate) fn watch_location_inner(
    &self,
    _options: WatchOptions,
    _callback_channel: Channel,
  ) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("watchLocation", WatchPayload { options: _options, channel: _callback_channel })
      .map_err(Into::into)
  }


  pub fn watch_direction<F: Fn(WatchEvent) + Send + Sync + 'static>(&self, options: WatchOptions, callback: F) -> crate::Result<u32> {
    let channel = Channel::new(move |event| {
      let payload = match event {
        InvokeResponseBody::Json(payload) => serde_json::from_str::<WatchEvent>(&payload)
          .unwrap_or_else(|error| {
            WatchEvent::Error(format!(
              "Couldn't deserialize watch event payload: `{error}`"
            ))
          }),
        _ => WatchEvent::Error("Unexpected watch event payload.".to_string()),
      };

      callback(payload);

      Ok(())
    });
    let id = channel.id();

    self.watch_direction_inner(options, channel)?;
    Ok(id)
  }

  pub(crate) fn watch_direction_inner(
    &self,
    _options: WatchOptions,
    _callback_channel: Channel,
  ) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("watchDirection", WatchPayload { options: _options, channel: _callback_channel })
      .map_err(Into::into)
  }


  pub fn add_task(&self, payload: AddTaskPayload) -> crate::Result<TxMapResponse> {
    self
      .0
      .run_mobile_plugin("addTask", payload)
      .map_err(Into::into)
  }


  pub fn clear_task(&self, id: String) -> crate::Result<TxMapResponse> {
    self
      .0
      .run_mobile_plugin("clearTask", ClearTaskPayload { id })
      .map_err(Into::into)
  }

  pub fn clear_channel(&self, id: u32) -> crate::Result<TxMapResponse> {
    self
      .0
      .run_mobile_plugin("clearChannel", ClearChannelPayload { id })
      .map_err(Into::into)
  }
  pub fn clear_watch_location(&self, id: u32) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("clearWatchLocation", ClearChannelPayload { id })
      .map_err(Into::into)
  }
  pub fn clear_watch_direction(&self, id: u32) -> crate::Result<()> {
    self
      .0
      .run_mobile_plugin("clearWatchDirection", ClearChannelPayload { id })
      .map_err(Into::into)
  }
}


