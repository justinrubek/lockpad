use crate::error::Error;
use axum::extract::{rejection::FormRejection, Form, FromRequest, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[async_trait::async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}
