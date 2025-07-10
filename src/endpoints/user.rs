//! User/Profile API endpoints for the Canva Connect API.
//!
//! This module provides access to user account information and capabilities including:
//! - Getting basic user identification (user ID, team ID)
//! - Retrieving user profile information (display name)
//! - Checking user capabilities for advanced features
//!
//! ## Available Operations
//!
//! | Operation | Method | Endpoint | Required Scope | Description |
//! |-----------|---------|----------|----------------|-------------|
//! | [`get_me`](UserApi::get_me) | `GET` | `/v1/users/me` | None | Get basic user identification |
//! | [`get_profile`](UserApi::get_profile) | `GET` | `/v1/users/me/profile` | `profile:read` | Get user profile information |
//! | [`get_capabilities`](UserApi::get_capabilities) | `GET` | `/v1/users/me/capabilities` | `profile:read` | Get user capabilities |
//!
//! ## OAuth Scopes
//!
//! - **No scope required** - Basic user identification (user ID, team ID)
//! - **`profile:read`** - Required for accessing user profile and capabilities
//!
//! ## User Capabilities
//!
//! Capabilities determine which advanced API endpoints a user can access:
//! - **`autofill`** - Autofill APIs (Canva Enterprise users)
//! - **`brand_template`** - Brand template APIs (Canva Enterprise users)
//! - **`resize`** - Design resize APIs (Canva Pro+ users)

use crate::{client::Client, error::Result};
use serde::{Deserialize, Serialize};

/// User API client
#[derive(Debug, Clone)]
pub struct UserApi {
    client: Client,
}

impl UserApi {
    /// Create a new user API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get basic user identification (user ID and team ID)
    ///
    /// **Required OAuth scope:** None (always accessible)
    pub async fn get_me(&self) -> Result<TeamUserSummary> {
        let response: UsersMeResponse = self.client.get_json("/v1/users/me").await?;
        Ok(response.team_user)
    }

    /// Get user profile information
    ///
    /// **Required OAuth scope:** `profile:read`
    pub async fn get_profile(&self) -> Result<UserProfile> {
        let response: UserProfileResponse = self.client.get_json("/v1/users/me/profile").await?;
        Ok(response.profile)
    }

    /// Get user capabilities
    ///
    /// **Required OAuth scope:** `profile:read`
    pub async fn get_capabilities(&self) -> Result<Vec<Capability>> {
        let response: GetUserCapabilitiesResponse =
            self.client.get_json("/v1/users/me/capabilities").await?;
        Ok(response.capabilities)
    }
}

/// Basic user identification containing user ID and team ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUserSummary {
    /// The ID of the user
    pub user_id: String,
    /// The ID of the user's Canva Team
    pub team_id: String,
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// The name of the user as shown in the Canva UI
    pub display_name: String,
}

/// User capabilities that determine access to advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Capability {
    /// Capability required to call autofill APIs
    /// Users that are members of a Canva Enterprise organization have this capability
    Autofill,
    /// Capability required to use brand template APIs
    /// Users that are members of a Canva Enterprise organization have this capability
    #[serde(rename = "brand_template")]
    BrandTemplate,
    /// Capability required to create design resize jobs
    /// Users on a Canva plan with premium features (such as Canva Pro) have this capability
    Resize,
}

/// Response from the users/me endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersMeResponse {
    /// Basic user identification
    pub team_user: TeamUserSummary,
}

/// Response from the users/me/profile endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    /// User profile information
    pub profile: UserProfile,
}

/// Response from the users/me/capabilities endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserCapabilitiesResponse {
    /// List of user capabilities
    pub capabilities: Vec<Capability>,
}

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Capability::Autofill => write!(f, "autofill"),
            Capability::BrandTemplate => write!(f, "brand_template"),
            Capability::Resize => write!(f, "resize"),
        }
    }
}
