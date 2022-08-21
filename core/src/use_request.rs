use dioxus::core::ScopeState;
use dioxus::hooks::{use_future, UseFutureDep};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;

pub fn use_request<'a, D, U, R>(cx: &'a ScopeState, dependencies: D, url: U) -> RequestResult<&R>
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
pub enum RequestResult<'a, T> {
    Success(T),
    Failure(&'a Error),
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
