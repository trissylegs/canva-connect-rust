//! Main client for the Canva Connect API

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
    pub fn new(access_token: AccessToken) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&access_token.authorization_header()).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("canva-connect-rust/0.1.0"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            http_client,
            base_url: BASE_URL.to_string(),
            access_token,
            rate_limiter: Arc::new(ApiRateLimiter::default()),
        }
    }

    /// Create a new client with a custom base URL and access token
    pub fn with_base_url(base_url: impl Into<String>, access_token: AccessToken) -> Self {
        let mut client = Self::new(access_token);
        client.base_url = base_url.into();
        client
    }

    /// Create a new client with a custom rate limiter
    pub fn with_rate_limiter(access_token: AccessToken, rate_limiter: ApiRateLimiter) -> Self {
        let mut client = Self::new(access_token);
        client.rate_limiter = Arc::new(rate_limiter);
        client
    }

    /// Get the assets API
    pub fn assets(&self) -> AssetsApi {
        AssetsApi::new(self.clone())
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

    /// Get the user API
    pub fn user(&self) -> UserApi {
        UserApi::new(self.clone())
    }

    /// Make a GET request
    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        self.request(reqwest::Method::GET, path, None::<&()>).await
    }

    /// Make a POST request
    pub async fn post<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<reqwest::Response> {
        self.request(reqwest::Method::POST, path, Some(body)).await
    }

    /// Make a PUT request
    pub async fn put<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<reqwest::Response> {
        self.request(reqwest::Method::PUT, path, Some(body)).await
    }

    /// Make a PATCH request
    pub async fn patch<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<reqwest::Response> {
        self.request(reqwest::Method::PATCH, path, Some(body)).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<reqwest::Response> {
        self.request(reqwest::Method::DELETE, path, None::<&()>).await
    }

    /// Make a request with optional body
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

        let response = request.send().await?;
        
        // Update rate limit info from headers
        let _rate_limit_info = RateLimitInfo::from_headers(response.headers());

        // Handle API errors
        if !response.status().is_success() {
            return self.handle_error_response(response).await;
        }

        Ok(response)
    }

    /// Handle error responses from the API
    async fn handle_error_response(&self, response: reqwest::Response) -> Result<reqwest::Response> {
        let status = response.status();
        
        // Try to parse API error
        if let Ok(api_error) = response.json::<ApiError>().await {
            return Err(Error::from(api_error));
        }

        // Fallback to generic HTTP error
        Err(Error::Generic(format!("HTTP {} error", status)))
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
        let client = Client::new(token);
        assert_eq!(client.base_url(), BASE_URL);
        assert_eq!(client.access_token().as_str(), "test-token");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let token = AccessToken::new("test-token");
        let base_url = "https://test.api.canva.com";
        let client = Client::with_base_url(base_url, token);
        assert_eq!(client.base_url(), base_url);
    }
}
