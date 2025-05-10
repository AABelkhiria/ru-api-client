use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError as UrlParseError;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("Configuration Error: Invalid base URL: {0}")]
    BaseUrlInvalid(#[from] UrlParseError),

    #[error("Configuration Error: Failed to build HTTP client: {0}")]
    HttpClientBuildFailed(reqwest::Error),

    #[error("Request Error: Failed to build or send the request: {0}")]
    RequestFailed(reqwest::Error),

    #[error("Network Error: Connection or timeout issue: {0}")]
    NetworkIssue(reqwest::Error),

    #[error("HTTP Error: Server responded with status {status}: {body}")]
    HttpError {
        // Server responded with non-2xx
        status: StatusCode,
        body: String,        // The raw error body from the server
        url: Option<String>, // Optionally include the URL that failed
    },

    #[error("Response Error: Failed to deserialize response body: {source}. Body snippet: '{body_snippet}'")]
    DeserializationFailed {
        // Failed to parse the 2xx response
        source: reqwest::Error, // The direct error from response.json()
        body_snippet: String,   // A snippet of the body for debugging
    },

    #[error("Client Internal Error: {0}")]
    InternalError(String),
}
