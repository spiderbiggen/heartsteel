#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use tracing::instrument;

pub use resources::*;

pub use crate::error::Error;

pub mod error;
pub mod version;
mod resources;

const RAW_COMMUNITY_DRAGON_URL: &str = "https://raw.communitydragon.org";
const CDN_COMMUNITY_DRAGON_URL: &str = "https://cdn.communitydragon.org";

#[cfg(feature = "client")]
pub mod client {
    use std::time::Duration;

    const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

    pub fn new() -> Result<reqwest::Client, reqwest::Error> {
        new_with_timeout(DEFAULT_TIMEOUT)
    }


    pub fn new_with_timeout(timeout: Duration) -> Result<reqwest::Client, reqwest::Error> {
        reqwest::Client::builder()
            .timeout(timeout)
            .build()
    }
}

#[instrument(skip_all, fields(http.uri = uri.as_str()), err)]
pub(crate) async fn get<U, T>(client: &reqwest::Client, uri: U) -> Result<T, Error>
    where
        U: IntoUrl,
        T: DeserializeOwned,
{
    let response = client.get(uri).send().await?;
    match response.error_for_status_ref() {
        Ok(_) => Ok(response.json().await?),
        Err(error) => {
            let body = response.text().await.ok();
            Err(Error::InvalidStatus { error, body })
        }
    }
}

