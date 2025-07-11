//! Authentication types and utilities for the Canva Connect API
//!
//! This module provides OAuth 2.0 authentication support for the Canva Connect API,
//! including access token management and OAuth scope documentation.

pub mod scopes;

use crate::error::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

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
    /// Generate new PKCE parameters
    pub fn new() -> Self {
        let code_verifier = Self::generate_code_verifier();
        let code_challenge = Self::generate_code_challenge(&code_verifier);

        Self {
            code_verifier,
            code_challenge,
        }
    }

    /// Generate a cryptographically secure code verifier (43-128 characters)
    fn generate_code_verifier() -> String {
        let mut rng = thread_rng();
        let length = rng.gen_range(43..=128);

        // Generate random bytes and encode as base64url
        let mut bytes = vec![0u8; length * 3 / 4]; // Enough bytes for desired length
        rng.fill(&mut bytes[..]);

        let verifier = URL_SAFE_NO_PAD.encode(&bytes);

        // Ensure we have the right length
        if verifier.len() >= 43 && verifier.len() <= 128 {
            verifier
        } else {
            // Fallback: generate exactly 43 characters
            let mut bytes = vec![0u8; 32]; // 32 bytes -> 43 chars in base64url
            rng.fill(&mut bytes[..]);
            URL_SAFE_NO_PAD.encode(&bytes)
        }
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
#[derive(Debug, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

/// OAuth 2.0 client for handling the authorization flow
#[derive(Debug)]
pub struct OAuthClient {
    config: OAuthConfig,
    http_client: reqwest::Client,
}

impl OAuthClient {
    /// Create a new OAuth client
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
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

    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: &str) -> Result<TokenExchangeResponse> {
        self.exchange_code_with_pkce(code, None).await
    }

    /// Exchange authorization code for access token with PKCE
    pub async fn exchange_code_with_pkce(
        &self,
        code: &str,
        pkce: Option<&PkceParams>,
    ) -> Result<TokenExchangeResponse> {
        let mut form_data = vec![
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.config.redirect_uri.as_str()),
        ];

        // Add code_verifier if PKCE is used
        if let Some(pkce) = pkce {
            form_data.push(("code_verifier", &pkce.code_verifier));
        }

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/token")
            .form(&form_data)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenExchangeResponse = response.json().await?;
            Ok(token_response)
        } else {
            let error_text = response.text().await?;
            Err(Error::Auth(format!("Token exchange failed: {error_text}")))
        }
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
            "http://localhost:8080/callback",
            vec![Scope::DesignMetaRead, Scope::AssetRead],
        );

        let pkce = PkceParams::new();
        let url = config
            .authorization_url_with_pkce(Some("test-state"), &pkce)
            .expect("Failed to generate authorization URL");

        assert!(url.contains("client_id=test-client-id"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fcallback"));
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
            redirect_uri: "http://localhost:8080/callback".to_string(),
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
            redirect_uri: "http://localhost:8080/callback".to_string(),
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
            "http://localhost:8080/callback",
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
