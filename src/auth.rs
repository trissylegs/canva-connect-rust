//! Authentication types and utilities for the Canva Connect API

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
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
        let mut url = url::Url::parse("https://www.canva.com/api/oauth/authorize")?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", &self.scopes_string());

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

    /// Get the authorization URL
    pub fn authorization_url(&self, state: Option<&str>) -> Result<String> {
        self.config.authorization_url(state)
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(&self, code: &str) -> Result<TokenExchangeResponse> {
        let request = TokenExchangeRequest {
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            code: code.to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: self.config.redirect_uri.clone(),
        };

        let response = self
            .http_client
            .post("https://api.canva.com/rest/v1/oauth/token")
            .json(&request)
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
