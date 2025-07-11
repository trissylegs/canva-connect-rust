//! Authentication types and utilities for the Canva Connect API
//!
//! This module provides comprehensive OAuth 2.0 authentication support for the Canva Connect API,
//! including access token management, refresh token handling, token introspection, and revocation.
//!
//! ## Features
//!
//! - **Token Storage**: Thread-safe token storage with automatic expiry management
//! - **Auto-refresh**: Automatic token refresh when access tokens expire
//! - **Token Introspection**: Check token validity and metadata
//! - **Token Revocation**: Revoke access and refresh tokens
//! - **Thread Safety**: All operations are safe for concurrent use
//!
//! ## Basic Usage
//!
//! ```rust
//! use canva_connect::auth::{OAuthConfig, OAuthClient, Scope};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create OAuth configuration
//! let config = OAuthConfig::new(
//!     "your_client_id",
//!     "your_client_secret",
//!     "https://your-app.com/callback",
//!     vec![Scope::AssetRead, Scope::AssetWrite],
//! );
//!
//! // Create OAuth client with automatic token management
//! let client = OAuthClient::new(config);
//!
//! // Get authorization URL
//! let auth_url = client.authorization_url(Some("state"))?;
//!
//! // After user authorizes, exchange the code for tokens
//! let token_response = client.exchange_code("authorization_code").await?;
//!
//! // Get access token (automatically refreshes if expired)
//! let access_token = client.get_access_token().await?;
//!
//! // Check if current token is valid
//! let is_valid = client.is_token_valid().await;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Token Management
//!
//! ```rust
//! use canva_connect::auth::{OAuthClient, OAuthConfig, TokenStore, Scope};
//!
//! # async fn advanced_example() -> Result<(), Box<dyn std::error::Error>> {
//! # let config = OAuthConfig::new("id", "secret", "uri", vec![]);
//! # let client = OAuthClient::new(config.clone());
//! // Manually refresh tokens
//! let refreshed_tokens = client.refresh_token().await?;
//!
//! // Introspect a token
//! let introspection = client.introspect_token("token_to_check").await?;
//! if introspection.active {
//!     println!("Token is valid");
//! }
//!
//! // Revoke a token
//! client.revoke_token("token_to_revoke", Some("access_token")).await?;
//!
//! // Share token store between multiple clients
//! let shared_store = TokenStore::new();
//! let client1 = OAuthClient::with_token_store(config.clone(), shared_store.clone());
//! let client2 = OAuthClient::with_token_store(config, shared_store);
//!
//! # Ok(())
//! # }
//! ```

pub mod scopes;

use crate::error::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// OAuth 2.0 access token for authenticating with the Canva Connect API
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    token: String,
}

impl AccessToken {
    /// Create a new access token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    /// Get the token value
    pub fn as_str(&self) -> &str {
        &self.token
    }

    /// Get the authorization header value
    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bearer {}", self.token)
    }
}

impl From<String> for AccessToken {
    fn from(token: String) -> Self {
        Self::new(token)
    }
}

impl From<&str> for AccessToken {
    fn from(token: &str) -> Self {
        Self::new(token)
    }
}

/// OAuth 2.0 scopes for the Canva Connect API
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    /// Read asset metadata
    #[serde(rename = "asset:read")]
    AssetRead,
    /// Write assets
    #[serde(rename = "asset:write")]
    AssetWrite,
    /// Read brand template metadata
    #[serde(rename = "brandtemplate:meta:read")]
    BrandTemplateMetaRead,
    /// Read brand template content
    #[serde(rename = "brandtemplate:content:read")]
    BrandTemplateContentRead,
    /// Read comments
    #[serde(rename = "comment:read")]
    CommentRead,
    /// Write comments
    #[serde(rename = "comment:write")]
    CommentWrite,
    /// Read design metadata
    #[serde(rename = "design:meta:read")]
    DesignMetaRead,
    /// Read design content
    #[serde(rename = "design:content:read")]
    DesignContentRead,
    /// Write design content
    #[serde(rename = "design:content:write")]
    DesignContentWrite,
    /// Read folder metadata
    #[serde(rename = "folder:read")]
    FolderRead,
    /// Write folders
    #[serde(rename = "folder:write")]
    FolderWrite,
    /// Read profile information
    #[serde(rename = "profile:read")]
    ProfileRead,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scope_str = match self {
            Scope::AssetRead => "asset:read",
            Scope::AssetWrite => "asset:write",
            Scope::BrandTemplateMetaRead => "brandtemplate:meta:read",
            Scope::BrandTemplateContentRead => "brandtemplate:content:read",
            Scope::CommentRead => "comment:read",
            Scope::CommentWrite => "comment:write",
            Scope::DesignMetaRead => "design:meta:read",
            Scope::DesignContentRead => "design:content:read",
            Scope::DesignContentWrite => "design:content:write",
            Scope::FolderRead => "folder:read",
            Scope::FolderWrite => "folder:write",
            Scope::ProfileRead => "profile:read",
        };
        write!(f, "{scope_str}")
    }
}

/// PKCE (Proof Key for Code Exchange) parameters for OAuth 2.0
#[derive(Debug, Clone)]
pub struct PkceParams {
    /// Code verifier (43-128 characters)
    pub code_verifier: String,
    /// Code challenge (SHA256 hash of verifier, base64url encoded)
    pub code_challenge: String,
}

impl PkceParams {
    /// Generate new PKCE parameters with default length (43 characters, 256 bits of entropy)
    pub fn new() -> Self {
        Self::with_length(43)
    }

    /// Generate PKCE parameters with custom verifier length (43-128 characters)
    pub fn with_length(length: usize) -> Self {
        let code_verifier = Self::generate_code_verifier(length);
        let code_challenge = Self::generate_code_challenge(&code_verifier);

        Self {
            code_verifier,
            code_challenge,
        }
    }

    /// Generate a cryptographically secure code verifier with specified length
    fn generate_code_verifier(target_length: usize) -> String {
        let length = target_length.clamp(43, 128);
        let mut rng = thread_rng();

        // Generate enough random bytes to ensure we get the target length
        // Base64url encoding: 4 chars per 3 bytes, so we need (length * 3) / 4 bytes (rounded up)
        let byte_count = (length * 3).div_ceil(4);
        let mut bytes = vec![0u8; byte_count];
        rng.fill(&mut bytes[..]);

        let mut verifier = URL_SAFE_NO_PAD.encode(&bytes);

        // Truncate to exact length if needed
        verifier.truncate(length);
        verifier
    }

    /// Generate code challenge from verifier (SHA256 hash, base64url encoded)
    fn generate_code_challenge(verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let result = hasher.finalize();
        URL_SAFE_NO_PAD.encode(result)
    }
}

impl Default for PkceParams {
    fn default() -> Self {
        Self::new()
    }
}

/// OAuth 2.0 configuration for the Canva Connect API
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// Client ID from your Canva app
    pub client_id: String,
    /// Client secret from your Canva app
    pub client_secret: String,
    /// Redirect URI registered with your Canva app
    pub redirect_uri: String,
    /// OAuth 2.0 scopes to request
    pub scopes: Vec<Scope>,
}

impl OAuthConfig {
    /// Create a new OAuth configuration
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
        scopes: Vec<Scope>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            scopes,
        }
    }

    /// Generate the authorization URL for the OAuth flow
    pub fn authorization_url(&self, state: Option<&str>) -> Result<String> {
        let pkce = PkceParams::new();
        self.authorization_url_with_pkce(state, &pkce)
    }

    /// Generate the authorization URL with PKCE parameters
    pub fn authorization_url_with_pkce(
        &self,
        state: Option<&str>,
        pkce: &PkceParams,
    ) -> Result<String> {
        let mut url = url::Url::parse("https://www.canva.com/api/oauth/authorize")?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &self.scopes_string())
            .append_pair("code_challenge", &pkce.code_challenge)
            .append_pair("code_challenge_method", "S256");

        if let Some(state) = state {
            url.query_pairs_mut().append_pair("state", state);
        }

        Ok(url.to_string())
    }

    /// Convert scopes to a space-separated string
    pub fn scopes_string(&self) -> String {
        self.scopes
            .iter()
            .map(|scope| scope.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Token exchange request for OAuth 2.0
#[derive(Debug, Serialize)]
pub struct TokenExchangeRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
}

/// Token exchange response from OAuth 2.0
#[derive(Debug, Clone, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// Represents a complete OAuth 2.0 token set with expiry information
#[derive(Debug, Clone)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_at: Option<Instant>,
    pub scope: Option<String>,
}

impl TokenSet {
    /// Create a new token set from a token exchange response
    pub fn from_exchange_response(response: TokenExchangeResponse) -> Self {
        let expires_at = response
            .expires_in
            .map(|expires_in| Instant::now() + Duration::from_secs(expires_in));

        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            token_type: response.token_type,
            expires_at,
            scope: response.scope,
        }
    }

    /// Check if the access token is expired or will expire soon
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|expires_at| Instant::now() >= expires_at)
            .unwrap_or(false)
    }

    /// Check if the access token will expire within the given duration
    pub fn expires_within(&self, duration: Duration) -> bool {
        self.expires_at
            .map(|expires_at| Instant::now() + duration >= expires_at)
            .unwrap_or(false)
    }

    /// Get the access token as an AccessToken instance
    pub fn access_token(&self) -> AccessToken {
        AccessToken::new(&self.access_token)
    }
}

/// Thread-safe token storage for OAuth 2.0 tokens
#[derive(Debug, Clone)]
pub struct TokenStore {
    tokens: Arc<RwLock<Option<TokenSet>>>,
}

impl TokenStore {
    /// Create a new empty token store
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(None)),
        }
    }

    /// Store a token set
    pub async fn store(&self, token_set: TokenSet) {
        let mut tokens = self.tokens.write().await;
        *tokens = Some(token_set);
    }

    /// Get the current token set
    pub async fn get(&self) -> Option<TokenSet> {
        let tokens = self.tokens.read().await;
        tokens.clone()
    }

    /// Get the current access token if available and not expired
    pub async fn get_valid_access_token(&self) -> Option<AccessToken> {
        let tokens = self.tokens.read().await;
        if let Some(token_set) = tokens.as_ref() {
            if !token_set.is_expired() {
                return Some(token_set.access_token());
            }
        }
        None
    }

    /// Check if we have a valid refresh token
    pub async fn has_refresh_token(&self) -> bool {
        let tokens = self.tokens.read().await;
        tokens
            .as_ref()
            .and_then(|t| t.refresh_token.as_ref())
            .is_some()
    }

    /// Clear all stored tokens
    pub async fn clear(&self) {
        let mut tokens = self.tokens.write().await;
        *tokens = None;
    }
}

impl Default for TokenStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Token refresh request for OAuth 2.0
#[derive(Debug, Serialize)]
pub struct TokenRefreshRequest {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub grant_type: String,
}

/// Token introspection request
#[derive(Debug, Serialize)]
pub struct TokenIntrospectionRequest {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
}

/// Token introspection response
#[derive(Debug, Deserialize)]
pub struct TokenIntrospectionResponse {
    pub active: bool,
    pub exp: Option<u64>,
    pub scope: Option<String>,
    pub client_id: Option<String>,
    pub username: Option<String>,
}

/// Token revocation request
#[derive(Debug, Serialize)]
pub struct TokenRevocationRequest {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
    pub token_type_hint: Option<String>,
}

/// OAuth 2.0 client for handling the authorization flow with token management
#[derive(Debug, Clone)]
pub struct OAuthClient {
    config: OAuthConfig,
    http_client: reqwest::Client,
    token_store: TokenStore,
}

impl OAuthClient {
    /// Create a new OAuth client
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
            token_store: TokenStore::new(),
        }
    }

    /// Create a new OAuth client with a custom token store
    pub fn with_token_store(config: OAuthConfig, token_store: TokenStore) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
            token_store,
        }
    }

    /// Get the authorization URL (with PKCE enabled by default)
    pub fn authorization_url(&self, state: Option<&str>) -> Result<(String, PkceParams)> {
        let pkce = PkceParams::new();
        let url = self.config.authorization_url_with_pkce(state, &pkce)?;
        Ok((url, pkce))
    }

    /// Get the authorization URL with specific PKCE parameters
    pub fn authorization_url_with_pkce(
        &self,
        state: Option<&str>,
        pkce: &PkceParams,
    ) -> Result<String> {
        self.config.authorization_url_with_pkce(state, pkce)
    }

    /// Exchange authorization code for access token (PKCE required for Canva Connect API)
    ///
    /// Note: This method is deprecated. Use `exchange_code_with_pkce` instead as PKCE is required.
    #[deprecated(
        note = "PKCE is required for Canva Connect API. Use exchange_code_with_pkce instead."
    )]
    pub async fn exchange_code(&self, _code: &str) -> Result<TokenExchangeResponse> {
        Err(Error::Auth(
            "PKCE is required for Canva Connect API. Use exchange_code_with_pkce instead."
                .to_string(),
        ))
    }

    /// Exchange authorization code for access token with PKCE and store it
    ///
    /// PKCE is required for the Canva Connect API.
    pub async fn exchange_code_with_pkce(
        &self,
        code: &str,
        pkce: &PkceParams,
    ) -> Result<TokenExchangeResponse> {
        let form_data = vec![
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.config.redirect_uri.as_str()),
            ("code_verifier", &pkce.code_verifier),
        ];

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/token")
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenExchangeResponse = response.json().await?;

            // Store the tokens
            let token_set = TokenSet::from_exchange_response(token_response.clone());
            self.token_store.store(token_set).await;

            Ok(token_response)
        } else {
            let error_text = response.text().await?;
            Err(Error::Auth(format!("Token exchange failed: {error_text}")))
        }
    }

    /// Get a valid access token, refreshing if necessary
    pub async fn get_access_token(&self) -> Result<AccessToken> {
        // First, try to get a valid non-expired token
        if let Some(token) = self.token_store.get_valid_access_token().await {
            return Ok(token);
        }

        // If no valid token, try to refresh
        if self.token_store.has_refresh_token().await {
            self.refresh_token().await?;
            return self
                .token_store
                .get_valid_access_token()
                .await
                .ok_or_else(|| {
                    Error::Auth("Failed to get access token after refresh".to_string())
                });
        }

        Err(Error::Auth(
            "No valid access token available and no refresh token".to_string(),
        ))
    }

    /// Refresh the access token using the refresh token
    pub async fn refresh_token(&self) -> Result<TokenExchangeResponse> {
        let current_tokens = self
            .token_store
            .get()
            .await
            .ok_or_else(|| Error::Auth("No tokens available for refresh".to_string()))?;

        let refresh_token = current_tokens
            .refresh_token
            .ok_or_else(|| Error::Auth("No refresh token available".to_string()))?;

        let request = TokenRefreshRequest {
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            refresh_token,
            grant_type: "refresh_token".to_string(),
        };

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/token")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenExchangeResponse = response.json().await?;

            // Store the new tokens
            let token_set = TokenSet::from_exchange_response(token_response.clone());
            self.token_store.store(token_set).await;

            Ok(token_response)
        } else {
            let error_text = response.text().await?;
            Err(Error::Auth(format!("Token refresh failed: {error_text}")))
        }
    }

    /// Introspect a token to check its validity and metadata
    pub async fn introspect_token(&self, token: &str) -> Result<TokenIntrospectionResponse> {
        let request = TokenIntrospectionRequest {
            token: token.to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
        };

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/introspect")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let introspection_response: TokenIntrospectionResponse = response.json().await?;
            Ok(introspection_response)
        } else {
            let error_text = response.text().await?;
            Err(Error::Auth(format!(
                "Token introspection failed: {error_text}"
            )))
        }
    }

    /// Revoke a token (access or refresh token)
    pub async fn revoke_token(&self, token: &str, token_type_hint: Option<&str>) -> Result<()> {
        let request = TokenRevocationRequest {
            token: token.to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            token_type_hint: token_type_hint.map(|s| s.to_string()),
        };

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/revoke")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            // Clear stored tokens if we revoked the current access token
            if let Some(current_tokens) = self.token_store.get().await {
                if current_tokens.access_token == token {
                    self.token_store.clear().await;
                }
            }
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(Error::Auth(format!(
                "Token revocation failed: {error_text}"
            )))
        }
    }

    /// Get the current token store
    pub fn token_store(&self) -> &TokenStore {
        &self.token_store
    }

    /// Check if the current token is valid (not expired)
    pub async fn is_token_valid(&self) -> bool {
        self.token_store.get_valid_access_token().await.is_some()
    }

    /// Clear all stored tokens
    pub async fn clear_tokens(&self) {
        self.token_store.clear().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_params_generation() {
        let pkce = PkceParams::new();

        // Code verifier should be 43-128 characters
        assert!(pkce.code_verifier.len() >= 43);
        assert!(pkce.code_verifier.len() <= 128);

        // Code challenge should be base64url encoded (43 chars for SHA256)
        assert_eq!(pkce.code_challenge.len(), 43);

        // Code challenge should be different from verifier
        assert_ne!(pkce.code_verifier, pkce.code_challenge);
    }

    #[test]
    fn test_pkce_code_challenge_deterministic() {
        let verifier = "test-verifier-123456789012345678901234567890123";
        let challenge = PkceParams::generate_code_challenge(verifier);

        // Should produce same challenge for same verifier
        assert_eq!(challenge, PkceParams::generate_code_challenge(verifier));

        // Should be base64url encoded
        assert_eq!(challenge.len(), 43);
        assert!(!challenge.contains('='));
        assert!(!challenge.contains('+'));
        assert!(!challenge.contains('/'));
    }

    #[test]
    fn test_pkce_verifier_uniqueness() {
        let pkce1 = PkceParams::new();
        let pkce2 = PkceParams::new();

        // Each generation should produce unique verifiers
        assert_ne!(pkce1.code_verifier, pkce2.code_verifier);
        assert_ne!(pkce1.code_challenge, pkce2.code_challenge);
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_oauth_config_authorization_url_with_pkce() {
        let config = OAuthConfig::new(
            "test-client-id",
            "test-client-secret",
            "http://127.0.0.1:8080/callback",
            vec![Scope::DesignMetaRead, Scope::AssetRead],
        );

        let pkce = PkceParams::new();
        let url = config
            .authorization_url_with_pkce(Some("test-state"), &pkce)
            .expect("Failed to generate authorization URL");

        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A8080%2Fcallback"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("scope=design%3Ameta%3Aread+asset%3Aread"));
        assert!(url.contains("state=test-state"));
        assert!(url.contains(&format!("code_challenge={}", pkce.code_challenge)));
        assert!(url.contains("code_challenge_method=S256"));
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_token_exchange_request_with_pkce() {
        let pkce = PkceParams::new();
        let request = TokenExchangeRequest {
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            code: "test-code".to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "http://127.0.0.1:8080/callback".to_string(),
            code_verifier: Some(pkce.code_verifier.clone()),
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize request");
        assert!(json.contains(&format!("\"code_verifier\":\"{}\"", pkce.code_verifier)));
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_token_exchange_request_without_pkce() {
        let request = TokenExchangeRequest {
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            code: "test-code".to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "http://127.0.0.1:8080/callback".to_string(),
            code_verifier: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize request");
        assert!(!json.contains("code_verifier"));
    }

    #[test]
    #[allow(clippy::expect_used)]
    fn test_oauth_client_authorization_url_returns_pkce() {
        let config = OAuthConfig::new(
            "test-client-id",
            "test-client-secret",
            "http://127.0.0.1:8080/callback",
            vec![Scope::DesignMetaRead],
        );

        let client = OAuthClient::new(config);
        let (url, pkce) = client
            .authorization_url(None)
            .expect("Failed to generate authorization URL");

        assert!(url.contains("code_challenge"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(pkce.code_verifier.len() >= 43);
        assert!(pkce.code_verifier.len() <= 128);
    }
}
