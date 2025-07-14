use crate::api::error::ApiError;
use axum::extract::rejection::{FormRejection, JsonRejection, PathRejection};
use axum::extract::{FromRequest, Path, Request};
use axum::{Form, Json};
use serde::de::DeserializeOwned;
use validator::Validate;
// use crate::ap

/// 如果是get请求, Form等同于Query, 因此不需要定义 `ValidatedQuery`
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedPath<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
  Form<T>: FromRequest<S, Rejection = FormRejection>,
{
  type Rejection = ApiError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Form(value) = Form::<T>::from_request(req, state).await?; // 如果是get方法, Form等同于Query
    value.validate()?;
    Ok(ValidatedForm(value))
  }
}

impl<T, S> FromRequest<S> for ValidatedPath<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
  Path<T>: FromRequest<S, Rejection = PathRejection>,
{
  type Rejection = ApiError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Path(value) = Path::<T>::from_request(req, state).await?; // 如果是get方法, Form等同于Query
    value.validate()?;
    Ok(ValidatedPath(value))
  }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
  T: DeserializeOwned + Validate,
  S: Send + Sync,
  Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
  type Rejection = ApiError;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state).await?; // 如果是get方法, Form等同于Query
    value.validate()?;
    Ok(ValidatedJson(value))
  }
}
