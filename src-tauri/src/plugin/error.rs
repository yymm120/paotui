// Result

pub type Result<T> = core::result::Result<T, Error>;

// Error
#[derive(Debug)]
pub enum Error {
  StoreError(tauri_plugin_store::Error),
  HttpError(tauri_plugin_http::reqwest::Error),
}

impl From<tauri_plugin_store::Error> for Error {
  fn from(val: tauri_plugin_store::Error) -> Self {
    Error::StoreError(val)
  }
}
impl From<tauri_plugin_http::reqwest::Error> for Error {
  fn from(val: tauri_plugin_http::reqwest::Error) -> Self {
    Error::HttpError(val)
  }
}
impl core::fmt::Display for Error {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
    write!(fmt, "{self:?}")
  }
}

impl std::error::Error for Error {}
