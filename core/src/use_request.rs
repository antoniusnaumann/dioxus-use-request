use dioxus::core::ScopeState;
use dioxus::hooks::{use_future, UseFutureDep};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

/// HTTP request methods that are supported by this crate
use RequestMethod::*;
pub enum RequestMethod {
    GET,
    // HEAD,
    // POST,
    // PUT,
    // DELETE,
    // CONNECT,
    // OPTIONS,
    // TRACE,
    // PATCH
}

/// Performs a request of the specified method
pub fn use_request<'a, D, U, R>(
    cx: &'a ScopeState,
    dependencies: D,
    url: U,
    // TODO: Payload
    method: RequestMethod,
) -> RequestResult<&R>
where
    D: UseFutureDep,
    U: 'static + IntoUrl,
    R: 'static + DeserializeOwned,
{
    match method {
        GET => use_get(cx, dependencies, url),
    }
}

/// Dioxus hook to perform a simple get request
pub fn use_get<'a, D, U, R>(cx: &'a ScopeState, dependencies: D, url: U) -> RequestResult<&R>
where
    D: UseFutureDep,
    U: 'static + IntoUrl,
    R: 'static + DeserializeOwned,
{
    use_future(cx, dependencies, |_| async move {
        reqwest::get(url).await.unwrap().json::<R>().await
    })
    .value()
    .into()
}

type Error = reqwest::Error;

pub use RequestResult::*;
/// Enum type that indicates the current status of a request
pub enum RequestResult<'a, T> {
    /// Result of a successful request that contains the returned value of type T
    Success(T),
    /// Result of a failed request that contains an error describing why the request failed
    Failure(&'a Error),
    /// Enum variant to represent a pending request that did not produce a result yet
    Loading,
}

impl<'a, T> From<Option<&'a Result<T, Error>>> for RequestResult<'a, &'a T> {
    fn from(result: Option<&'a Result<T, Error>>) -> Self {
        match result {
            Some(Ok(val)) => Success(val),
            Some(Err(err)) => Failure(err),
            None => Loading,
        }
    }
}
