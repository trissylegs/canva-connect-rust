#![deny(clippy::unwrap_used, clippy::expect_used)]

//! # Canva Connect API Client
//!
//! A Rust client library for the Canva Connect API that provides a safe and ergonomic interface
//! for interacting with Canva's design platform.
//!
//! ## Features
//!
//! - **Async/await support** - Built on `tokio` and `reqwest`
//! - **Type safety** - Strongly typed API with comprehensive error handling
//! - **OAuth 2.0 authentication** - Full support for Canva's OAuth flow
//! - **Rate limiting** - Built-in rate limiting to respect API quotas
//! - **Async job handling** - Support for long-running operations like uploads and exports
//! - **Enterprise features** - Support for brand templates and autofill APIs
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! canva-connect = "0.1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))
//!         .expect("Failed to create client");
//!     
//!     // Create an asset upload job
//!     let metadata = canva_connect::endpoints::assets::AssetUploadMetadata::new(
//!         "my-image.png",
//!         vec!["design".to_string()]
//!     );
//!     let upload_job = client.assets().create_upload_job(vec![], metadata).await?;
//!     println!("Created upload job: {}", upload_job.id);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The client supports OAuth 2.0 authentication flow:
//!
//! ```rust,no_run
//! use canva_connect::auth::AccessToken;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // From environment variable
//! let token = AccessToken::new(&std::env::var("CANVA_ACCESS_TOKEN")?);
//!
//! // Or directly
//! let token = AccessToken::new("your-access-token");
//! # Ok(())
//! # }
//! ```
//!
//! ## API Reference
//!
//! This client provides access to all major Canva Connect API endpoints:
//!
//! ### Core Endpoints
//! - **[`endpoints::assets`]** - Upload, manage, and retrieve design assets
//!   - Upload assets from files or URLs
//!   - Get asset metadata and thumbnails  
//!   - Update asset names and tags
//!   - Delete assets from library
//! - **[`endpoints::designs`]** - Create and manage Canva designs
//!   - List user's designs with search and filtering
//!   - Get design metadata and URLs
//!   - Create new designs from presets or custom dimensions
//! - **[`endpoints::user`]** - User profile and account information
//!   - Get user profile details
//!   - Check user capabilities and features
//!   - User identification and verification
//!
//! ### Enterprise Endpoints (Coming Soon)
//! - **Brand Templates** - Work with brand templates and corporate designs
//! - **Autofill** - Automatically populate templates with data
//! - **Folders** - Organize content in folders and collections
//! - **Comments** - Add and manage comments on designs
//! - **Exports** - Export designs to various formats (PDF, PNG, etc.)
//!
//! ## OAuth Scopes
//!
//! Different API operations require different OAuth scopes. See [`auth::scopes`] for
//! a complete list of available scopes and their usage.
//!
//! Common scopes include:
//! - `asset:read` - Read access to user's assets
//! - `asset:write` - Write access to user's assets  
//! - `design:meta:read` - Read access to design metadata
//! - `design:content:read` - Read access to design content
//! - `design:content:write` - Write access to design content
//!
//! ## Examples
//!
//! ### Asset Upload from File
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken, endpoints::assets::AssetUploadMetadata};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"))
//!     .expect("Failed to create client");
//!
//! // Upload an asset
//! let metadata = AssetUploadMetadata::new("logo.png", vec!["branding".to_string()]);
//! let job = client.assets().create_upload_job(vec![], metadata).await?;
//!
//! // Wait for completion
//! let completed_job = client.assets().wait_for_upload_job(&job.id).await?;
//! println!("Asset uploaded: {:?}", completed_job);
//! # Ok(())
//! # }
//! ```
//!
//! ### Asset Upload from URL
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken, endpoints::assets::CreateUrlAssetUploadJobRequest};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"))
//!     .expect("Failed to create client");
//!
//! // Upload from URL
//! let request = CreateUrlAssetUploadJobRequest {
//!     url: "https://example.com/image.png".to_string(),
//!     name: "My Image".to_string(),
//! };
//! let job = client.assets().create_url_upload_job(request).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Create a Design
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::models::{CreateDesignRequest, DesignTypeInput, PresetDesignTypeName};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"))
//!     .expect("Failed to create client");
//!
//! // Create a presentation design
//! let request = CreateDesignRequest {
//!     design_type: Some(DesignTypeInput::Preset {
//!         name: PresetDesignTypeName::Presentation,
//!     }),
//!     title: Some("My Presentation".to_string()),
//!     asset_id: None,
//! };
//! let design = client.designs().create(request).await?;
//! println!("Created design: {}", design.design.id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Get User Profile
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"))
//!     .expect("Failed to create client");
//!
//! // Get user profile
//! let profile = client.user().get_profile().await?;
//! println!("User: {}", profile.display_name);
//! # Ok(())
//! # }
//! ```
//!
//! For more comprehensive examples, see the `examples/` directory in the repository:
//! - [`examples/asset_upload.rs`] - File-based asset upload with progress tracking
//! - [`examples/url_asset_upload.rs`] - URL-based asset upload with metadata updates
//! - [`examples/user_profile.rs`] - User profile and capabilities demonstration
//! - [`examples/observability.rs`] - OpenTelemetry tracing integration
//! - Design examples (coming soon) - Create and manage designs with various templates

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod observability;
pub mod rate_limit;

pub use client::Client;
pub use error::{Error, Result};
pub use models::*;

/// API version used by this client
pub const API_VERSION: &str = "v1";

/// Base URL for the Canva Connect API
pub const BASE_URL: &str = "https://api.canva.com/rest";
