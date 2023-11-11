#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::ops::Deref;
use std::time::Duration;

use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use tracing::instrument;

pub use crate::error::Error;

pub mod skins;
pub mod error;

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
}

const DEFAULT_TIMEOUT: u64 = 5;

impl Client {
    pub fn new() -> Result<Self, reqwest::Error> {
        Self::new_with_timeout(Duration::from_secs(DEFAULT_TIMEOUT))
    }


    pub fn new_with_timeout(timeout: Duration) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()?;
        Ok(Client { inner: client })
    }
}

impl Deref for Client {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[instrument(skip_all, fields(http.uri = uri.as_str()), err)]
pub async fn get<C, U, T>(client: C, uri: U) -> Result<T, Error>
    where
        C: AsRef<reqwest::Client>,
        U: IntoUrl,
        T: DeserializeOwned,
{
    let response = client.as_ref().get(uri).send().await?;
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.json().await?),
        Err(error) => {
            let body = response.text().await.ok();
            Err(Error::InvalidStatus { error, body })
        }
    }
}

