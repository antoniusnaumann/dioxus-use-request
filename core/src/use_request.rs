use dioxus::core::{ScopeState, TaskId};
use dioxus::hooks::{use_future, UseFutureDep};
use dioxus::prelude::UseFuture;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

/// Hook to perform a simple HTTP GET request in a Dioxus component.
///
/// Returns a `UseRequest` handle
pub fn use_request<D, U, R>(cx: &ScopeState, dependencies: D, url: U) -> UseRequest<R>
where
    D: UseFutureDep,
    U: 'static + IntoUrl,
    R: 'static + DeserializeOwned,
{
    let future = use_future(cx, dependencies, |_| async move {
        reqwest::get(url).await.unwrap().json::<R>().await
    });

    UseRequest { future }
}

/// Handle returned by the `use_request` hook that can be used to access the value, cancel or restart the underlying request.
pub struct UseRequest<'a, R> {
    future: &'a UseFuture<Result<R, Error>>,
}

impl<R> UseRequest<'_, R> {
    /// Restart the request without canceling the old request but ignoring it's output.
    pub fn restart(&self) {
        self.future.restart()
    }

    /// Return any value, even old values if this request has not completed yet.
    /// If the request did not finish yet, `Loading` is returned.
    pub fn value(&self) -> RequestResult<&R> {
        self.future.value().into()
    }

    /// Forcefully cancel the underlying future
    pub fn cancel(&self, cx: &ScopeState) {
        self.future.cancel(cx)
    }

    /// Get the ID of the underlying future in Dioxus' internal scheduler
    pub fn task(&self) -> Option<TaskId> {
        self.future.task()
    }
}

type Error = reqwest::Error;

pub use RequestResult::*;
/// Result type for the `use_request` hook
pub enum RequestResult<'a, T> {
    /// Result for a finished successful request with the requested value
    Success(T),
    /// Result for a finished failed request with the error that indicates why the request failed
    Failure(&'a Error),
    /// Symbolic enum field to indicate, that the request has not finished yet
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
