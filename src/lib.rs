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
//! ## Quick Start
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(AccessToken::new("your-access-token"));
//!     
//!     // List the user's designs
//!     let designs = client.designs().list().await?;
//!     println!("Found {} designs", designs.items.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod rate_limit;

pub use client::Client;
pub use error::{Error, Result};
pub use models::*;

/// API version used by this client
pub const API_VERSION: &str = "v1";

/// Base URL for the Canva Connect API
pub const BASE_URL: &str = "https://api.canva.com/rest";
