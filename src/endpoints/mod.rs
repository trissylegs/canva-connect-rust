//! API endpoint implementations for the Canva Connect API.
//!
//! This module contains the implementation for various Canva Connect API endpoints.
//! Each endpoint provides strongly typed methods for interacting with specific
//! parts of the Canva platform.
//!
//! ## Available Endpoints
//!
//! - [`assets`] - Upload, manage, and retrieve design assets
//! - [`designs`] - Create and manage Canva designs
//! - [`user`] - User profile and account information
//! - Folders - Organize content in folders (coming soon)
//! - Brand Templates - Work with brand templates (coming soon)
//! - Autofill - Use autofill functionality (coming soon)
//! - Comments - Add and manage comments (coming soon)
//! - Exports - Export designs to various formats (coming soon)
//!
//! ## Usage
//!
//! Access endpoints through the main [`Client`]:
//!
//! ```rust,no_run
//! use canva_connect::{Client, auth::AccessToken};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new(AccessToken::new("token"));
//!
//! // Access the assets endpoint
//! let assets = client.assets();
//! # Ok(())
//! # }
//! ```

pub mod assets;
pub mod designs;
pub mod user;

pub use assets::AssetsApi;
pub use designs::DesignsApi;
pub use user::UserApi;

// Stub implementations for other endpoints
use crate::client::Client;

macro_rules! stub_api {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            #[allow(dead_code)]
            client: Client,
        }

        impl $name {
            pub fn new(client: Client) -> Self {
                Self { client }
            }
        }
    };
}

stub_api!(AutofillApi);
stub_api!(BrandTemplatesApi);
stub_api!(CommentsApi);
stub_api!(ExportsApi);
stub_api!(FoldersApi);
