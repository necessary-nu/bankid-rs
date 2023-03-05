use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    error_code: String,
    details: String,
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(errorCode: {}, details: {})",
            self.error_code, self.details
        )
    }
}

pub use reqwest::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
