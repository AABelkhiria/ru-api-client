use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("HTTP client creation failed: {0}")]
    HttpClientCreationError(reqwest::Error),

    #[error("Request build or send failed: {0}")]
    RequestBuildOrSendFailed(reqwest::Error),

    #[error("Network error (connect/timeout): {0}")]
    NetworkError(reqwest::Error),

    #[error("API returned an error: Status={status}, Body='{body}'")]
    ApiError { status: StatusCode, body: String },

    #[error("Failed to deserialize response body: {source}. Body snippet: {body}")]
    DeserializationError { source: reqwest::Error, body: String },

    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),
}
