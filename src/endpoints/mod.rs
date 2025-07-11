//! API endpoint implementations for the Canva Connect API.
//!
//! This module contains the implementation for various Canva Connect API endpoints.
//! Each endpoint provides strongly typed methods for interacting with specific
//! parts of the Canva platform.
//!
//! ## Available Endpoints
//!
//! - [`assets`] - Upload, manage, and retrieve design assets
//! - [`autofill`] - Use autofill functionality with brand templates and data
//! - [`brand_templates`] - Work with brand templates and datasets
//! - [`comments`] - Add and manage comments and threads on designs
//! - [`designs`] - Create and manage Canva designs
//! - [`exports`] - Export designs to various file formats
//! - [`folders`] - Organize content in folders
//! - [`user`] - User profile and account information
//!
//! ## Usage
//!
//! Access endpoints through the main [`crate::Client`]:
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"))
//!     .expect("Failed to create client");
//!
//! // Access the assets endpoint
//! let assets = client.assets();
//! # Ok(())
//! # }
//! ```

pub mod assets;
pub mod autofill;
pub mod brand_templates;
pub mod comments;
pub mod designs;
pub mod exports;
pub mod folders;
pub mod user;

pub use assets::AssetsApi;
pub use autofill::AutofillApi;
pub use brand_templates::BrandTemplatesApi;
pub use comments::CommentsApi;
pub use designs::DesignsApi;
pub use exports::ExportsApi;
pub use folders::FoldersApi;
pub use user::UserApi;

// All endpoints are now implemented
