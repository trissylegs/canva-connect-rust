#![deny(clippy::unwrap_used, clippy::expect_used, clippy::unimplemented)]

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
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::endpoints::assets::AssetUploadMetadata;
//! use std::fs;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!     
//!     // Read file
//!     let file_data = fs::read("image.png")?;
//!     
//!     // Upload asset
//!     let metadata = AssetUploadMetadata::new("My Image", vec!["rust".to_string(), "upload".to_string()]);
//!     
//!     let upload_job = client.assets().create_upload_job(file_data, metadata).await?;
//!     let result = client.assets().wait_for_upload_job(&upload_job.id).await?;
//!     
//!     println!("Uploaded asset: {}", result.id);
//!     Ok(())
//! }
//! ```
//!
//! ### Asset Upload from URL
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::endpoints::assets::CreateUrlAssetUploadJobRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!     
//!     let request = CreateUrlAssetUploadJobRequest {
//!         url: "https://example.com/image.png".to_string(),
//!         name: "Image from URL".to_string(),
//!     };
//!     
//!     let upload_job = client.assets().create_url_upload_job(request).await?;
//!     let result = client.assets().wait_for_url_upload_job(&upload_job.id).await?;
//!     
//!     println!("Uploaded asset: {}", result.id);
//!     Ok(())
//! }
//! ```
//!
//! ### Get Asset Details
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!     
//!     // Get a specific asset by ID
//!     let asset = client.assets().get("asset-id").await?;
//!     println!("Asset: {} ({})", asset.name, asset.id);
//!     
//!     // Update asset metadata
//!     let update_request = canva_connect::endpoints::assets::UpdateAssetRequest {
//!         name: Some("Updated Asset Name".to_string()),
//!         tags: Some(vec!["rust".to_string(), "api".to_string()]),
//!     };
//!     
//!     let updated_asset = client.assets().update("asset-id", update_request).await?;
//!     println!("Updated asset: {}", updated_asset.name);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Create and Manage Folders
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::endpoints::folders::{CreateFolderRequest, UpdateFolderRequest, MoveFolderItemRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!     
//!     // Create a folder
//!     let create_request = CreateFolderRequest {
//!         name: "My Project".to_string(),
//!         parent_folder_id: "root".to_string(),
//!     };
//!     
//!     let folder_response = client.folders().create_folder(&create_request).await?;
//!     let folder = &folder_response.folder;
//!     println!("Created folder: {} (ID: {})", folder.name, folder.id);
//!     
//!     // List folder contents
//!     let list_request = canva_connect::endpoints::folders::ListFolderItemsRequest {
//!         limit: Some(50),
//!         continuation: None,
//!     };
//!     
//!     let items = client.folders().list_folder_items(&folder.id, &list_request).await?;
//!     println!("Found {} items in folder", items.items.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Export Designs
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//! use canva_connect::endpoints::exports::CreateDesignExportJobRequest;
//! use canva_connect::models::ExportFormat;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!     
//!     // Create export job
//!     let export_request = CreateDesignExportJobRequest {
//!         design_id: "design-id".to_string(),
//!         format: ExportFormat::Png {
//!             export_quality: None,
//!             height: None,
//!             width: None,
//!             pages: None,
//!         },
//!     };
//!     
//!     let export_job = client.exports().create_design_export_job(&export_request).await?;
//!     println!("Created export job: {}", export_job.job.id);
//!     
//!     // Get export job status
//!     let job_status = client.exports().get_design_export_job(&export_job.job.id).await?;
//!     println!("Export job status: {:?}", job_status.job.status);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"))?;
//!         
//!     match client.assets().get("invalid-id").await {
//!         Ok(asset) => println!("Asset: {}", asset.name),
//!         Err(Error::Api { code, message }) => {
//!             println!("API error {}: {}", code, message);
//!         }
//!         Err(Error::Http(e)) => {
//!             println!("HTTP error: {}", e);
//!         }
//!         Err(e) => {
//!             println!("Other error: {}", e);
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### Rate Limiting
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken, rate_limit::ApiRateLimiter};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let access_token = AccessToken::new("your-access-token");
//!     
//!     // Create client with custom rate limiter
//!     let rate_limiter = ApiRateLimiter::new(30); // 30 requests per minute
//!     let client = Client::with_rate_limiter(access_token, rate_limiter)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! For more comprehensive examples, see the `examples/` directory in the repository:
//! - [`examples/asset_upload.rs`] - File-based asset upload with progress tracking
//! - [`examples/url_asset_upload.rs`] - URL-based asset upload with metadata updates
//! - [`examples/user_profile.rs`] - User profile and capabilities demonstration
//! - [`examples/designs.rs`] - Create and manage designs with various templates
//! - [`examples/folders.rs`] - Create and organize content in folders
//! - [`examples/exports.rs`] - Export designs to various formats
//! - [`examples/observability.rs`] - OpenTelemetry tracing integration

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
