use crate::errors::ApiClientError;
use reqwest::header::HeaderMap;
use reqwest::{RequestBuilder, Response};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub struct ApiClient {
    base_url: Url,
    http_client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Result<Self, ApiClientError> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(ApiClientError::HttpClientBuildFailed)?;

        let base_url = Url::parse(base_url).map_err(ApiClientError::BaseUrlInvalid)?;

        Ok(ApiClient { base_url, http_client })
    }

    pub fn get(&self, endpoint: &str) -> Result<RequestBuilder, ApiClientError> {
        let url = self.build_url(endpoint)?;
        Ok(self.http_client.get(url))
    }

    pub fn build_url(&self, endpoint: &str) -> Result<Url, ApiClientError> {
        self.base_url.join(endpoint).map_err(ApiClientError::BaseUrlInvalid)
    }

    pub fn add_query_params(&self, builder: RequestBuilder, params: Option<&HashMap<&str, String>>) -> RequestBuilder {
        if let Some(query_params) = params {
            builder.query(query_params)
        } else {
            builder
        }
    }

    pub fn add_headers(&self, builder: RequestBuilder, headers: Option<&HeaderMap>) -> RequestBuilder {
        if let Some(custom_headers) = headers {
            builder.headers(custom_headers.clone())
        } else {
            builder
        }
    }

    pub async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        builder: RequestBuilder,
    ) -> Result<T, ApiClientError> {
        let response_result = builder.send().await;

        let response = match response_result {
            Ok(res) => res,
            Err(e) => {
                // More specific error for network/request phase issues
                if e.is_connect() || e.is_timeout() {
                    return Err(ApiClientError::NetworkIssue(e));
                }
                return Err(ApiClientError::RequestFailed(e));
            }
        };

        let response_status: reqwest::StatusCode = response.status();

        // handle status codes and deserialization
        if response_status.is_success() {
            return self.handle_json_response::<T>(response).await;
        } else if response_status.is_redirection() {
            // return a non supported error for redirection
            return Err(ApiClientError::InternalError(format!(
                "Redirection error: {}",
                response_status
            )));
        } else {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not read error body".to_string());
            Err(ApiClientError::HttpError {
                status: response_status,
                body: error_body,
                url: None,
            })
        }
    }

    async fn handle_json_response<T: serde::de::DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ApiClientError> {
        match response.json::<T>().await {
            Ok(result) => Ok(result),
            Err(err) => Err(ApiClientError::DeserializationFailed {
                source: err,
                body_snippet: String::from("Could not deserialize response"),
            }),
        }
    }
}
