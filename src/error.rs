#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request failed with: {error}")]
    InvalidStatus { #[source] error: reqwest::Error, body: Option<String> },
    #[error("{0}")]
    Timeout(#[source] reqwest::Error),
    #[error("{0}")]
    Reqwest(#[source] reqwest::Error),
    #[error(transparent)]
    Deserialize(#[from] serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        if value.is_timeout() {
            Error::Timeout(value)
        } else if value.is_status() {
            Error::InvalidStatus { error: value, body: None }
        } else {
            Error::Reqwest(value)
        }
    }
}