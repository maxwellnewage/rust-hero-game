use core::fmt;
use std::convert::From;

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
    DeserializeError(serde_json::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::RequestError(error)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::DeserializeError(error)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RequestError(error) => write!(f, "Request error: {}", error),
            ApiError::DeserializeError(error) => write!(f, "Deserialize error: {}", error),
        }
    }
}