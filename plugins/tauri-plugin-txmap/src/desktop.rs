use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use tauri::ipc::{Channel, InvokeResponseBody};
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Txmap<R>> {
  Ok(Txmap(app.clone()))
}

/// Access to the txmap APIs.
pub struct Txmap<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Txmap<R> {
  pub fn fragment(&self, payload: FragmentRequest) -> crate::Result<FragmentResponse> {
    Ok(FragmentResponse {
      status: None,
      message: None,
    })
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
    Ok(Default::default())
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
  ) -> crate::Result<TxMapResponse> {
    Ok(Default::default())
  }



  pub fn watch_direction<F: Fn(WatchEvent) + Send + Sync + 'static>(&self, options: WatchOptions, callback: F) -> crate::Result<u32> {
    println!("into desktop");
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
  ) -> crate::Result<TxMapResponse> {
    Ok(Default::default())
  }



  pub fn add_task(&self, payload: AddTaskPayload) -> crate::Result<TxMapResponse> {
    Ok(TxMapResponse {
      status: None,
      message: None,
    })
  }

  pub fn clear_task(&self, id: String) -> crate::Result<TxMapResponse> {
    Ok(TxMapResponse {
      status: None,
      message: None,
    })
  }
  pub fn clear_channel(&self, id: u32) -> crate::Result<TxMapResponse> {
    Ok(TxMapResponse {
      status: None,
      message: None,
    })
  }

  pub fn clear_watch_location(&self, id: u32) -> crate::Result<()> {
    Ok(())
  }

  pub fn clear_watch_direction(&self, id: u32) -> crate::Result<()> {
    Ok(())
  }
}