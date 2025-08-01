//! HTTP client implementation for the Canva Connect API.
//!
//! This module provides the main [`Client`] struct that handles all API communication,
//! including authentication, rate limiting, and endpoint access.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("your-token"))
//!     .expect("Failed to create client");
//! let assets_api = client.assets();
//! # Ok(())
//! # }
//! ```

use crate::{
    auth::AccessToken,
    endpoints::*,
    error::{ApiError, Error, Result},
    rate_limit::{ApiRateLimiter, RateLimitInfo},
    BASE_URL,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::sync::Arc;

/// Main client for the Canva Connect API
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    access_token: AccessToken,
    rate_limiter: Arc<ApiRateLimiter>,
}

impl Client {
    /// Create a new client with the given access token
    pub fn new(access_token: AccessToken) -> crate::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&access_token.authorization_header())?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("canva-connect-rust/0.1.0"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(crate::Error::ClientBuild)?;

        Ok(Self {
            http_client,
            base_url: BASE_URL.to_string(),
            access_token,
            rate_limiter: Arc::new(ApiRateLimiter::default()),
        })
    }

    /// Create a new client with a custom base URL and access token
    pub fn with_base_url(
        base_url: impl Into<String>,
        access_token: AccessToken,
    ) -> crate::Result<Self> {
        let mut client = Self::new(access_token)?;
        client.base_url = base_url.into();
        Ok(client)
    }

    /// Create a new client with a custom rate limiter
    pub fn with_rate_limiter(
        access_token: AccessToken,
        rate_limiter: ApiRateLimiter,
    ) -> crate::Result<Self> {
        let mut client = Self::new(access_token)?;
        client.rate_limiter = Arc::new(rate_limiter);
        Ok(client)
    }

    /// Get the assets API
    pub fn assets(&self) -> AssetsApi {
        AssetsApi::new(self.clone())
    }

    /// Get the user API
    pub fn user(&self) -> UserApi {
        UserApi::new(self.clone())
    }

    /// Get the designs API
    pub fn designs(&self) -> DesignsApi {
        DesignsApi::new(self.clone())
    }

    /// Get the folders API
    pub fn folders(&self) -> FoldersApi {
        FoldersApi::new(self.clone())
    }

    /// Get the brand templates API
    pub fn brand_templates(&self) -> BrandTemplatesApi {
        BrandTemplatesApi::new(self.clone())
    }

    /// Get the autofill API
    pub fn autofill(&self) -> AutofillApi {
        AutofillApi::new(self.clone())
    }

    /// Get the comments API
    pub fn comments(&self) -> CommentsApi {
        CommentsApi::new(self.clone())
    }

    /// Get the exports API
    pub fn exports(&self) -> ExportsApi {
        ExportsApi::new(self.clone())
    }

    /// Make a GET request
    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        self.request(reqwest::Method::GET, path, None::<&()>).await
    }

    /// Make a POST request
    pub async fn post<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        self.request(reqwest::Method::POST, path, Some(body)).await
    }

    /// Make a PUT request
    pub async fn put<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        self.request(reqwest::Method::PUT, path, Some(body)).await
    }

    /// Make a PATCH request
    pub async fn patch<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        self.request(reqwest::Method::PATCH, path, Some(body)).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<reqwest::Response> {
        self.request(reqwest::Method::DELETE, path, None::<&()>)
            .await
    }

    /// Make a request with optional body
    #[cfg_attr(feature = "observability", tracing::instrument(
        skip(self, body),
        fields(
            http.method = %method,
            http.url = %format!("{}{}", self.base_url, path),
            http.status_code = tracing::field::Empty,
            canva.api.path = path,
            canva.request_id = tracing::field::Empty,
        )
    ))]
    pub async fn request<T: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<reqwest::Response> {
        // Wait for rate limiting
        self.rate_limiter.wait_for_request().await;

        let url = format!("{}{}", self.base_url, path);
        let mut request = self.http_client.request(method, &url);

        if let Some(body) = body {
            request = request.json(body);
        }

        #[cfg(feature = "observability")]
        tracing::debug!("Sending HTTP request");

        let response = request.send().await?;

        // Record response status and request ID in span
        #[cfg(feature = "observability")]
        {
            tracing::Span::current().record("http.status_code", response.status().as_u16());

            // Capture x-request-id header for tracing correlation
            if let Some(request_id) = response.headers().get("x-request-id") {
                if let Ok(request_id_str) = request_id.to_str() {
                    tracing::Span::current().record("canva.request_id", request_id_str);
                    tracing::debug!("Canva API request ID: {}", request_id_str);
                }
            }
        }

        // Update rate limit info from headers
        let _rate_limit_info = RateLimitInfo::from_headers(response.headers());

        // Handle API errors
        if !response.status().is_success() {
            #[cfg(feature = "observability")]
            {
                let request_id = response
                    .headers()
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("unknown");
                tracing::warn!(
                    "HTTP request failed with status: {} (request_id: {})",
                    response.status(),
                    request_id
                );
            }
            return self.handle_error_response(response).await;
        }

        #[cfg(feature = "observability")]
        tracing::debug!("HTTP request completed successfully");

        Ok(response)
    }

    /// Handle error responses from the API
    async fn handle_error_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response> {
        let status = response.status();

        // Try to parse API error
        if let Ok(api_error) = response.json::<ApiError>().await {
            return Err(Error::from(api_error));
        }

        // Fallback to generic HTTP error
        Err(Error::Generic(format!("HTTP {status} error")))
    }

    /// Get a JSON response from a path
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.get(path).await?;
        let json = response.json().await?;
        Ok(json)
    }

    /// Post JSON and get JSON response
    pub async fn post_json<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let response = self.post(path, body).await?;
        let json = response.json().await?;
        Ok(json)
    }

    /// Patch JSON and get JSON response
    pub async fn patch_json<T: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let response = self.patch(path, body).await?;
        let json = response.json().await?;
        Ok(json)
    }

    /// Upload a file as multipart form data
    pub async fn upload_file(
        &self,
        path: &str,
        file_data: Vec<u8>,
        metadata: Option<&str>,
    ) -> Result<reqwest::Response> {
        // Wait for rate limiting
        self.rate_limiter.wait_for_request().await;

        let url = format!("{}{}", self.base_url, path);
        let mut request = self.http_client.post(&url);

        if let Some(metadata) = metadata {
            request = request.header("Asset-Upload-Metadata", metadata);
        }

        let response = request
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(file_data)
            .send()
            .await?;

        // Handle API errors
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }

        Ok(response)
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the access token
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    /// Get the HTTP client
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let token = AccessToken::new("test-token");
        #[allow(clippy::expect_used)]
        let client = Client::new(token).expect("Failed to create client");
        assert_eq!(client.base_url(), BASE_URL);
        assert_eq!(client.access_token().as_str(), "test-token");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let token = AccessToken::new("test-token");
        let base_url = "https://test.api.canva.com";
        #[allow(clippy::expect_used)]
        let client = Client::with_base_url(base_url, token).expect("Failed to create client");
        assert_eq!(client.base_url(), base_url);
    }
}
