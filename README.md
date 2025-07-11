# Canva Connect Rust Client

A Rust client library for the [Canva Connect API](https://www.canva.dev/docs/connect/) that provides a safe and ergonomic interface for interacting with Canva's design platform.

## Features

- **Complete API Coverage** - All 34 endpoints across 8 API modules with working examples
- **Async/await support** - Built on `tokio` and `reqwest`
- **Type safety** - Strongly typed API with comprehensive error handling
- **OAuth 2.0 authentication** - Full support for Canva's OAuth flow
- **Rate limiting** - Built-in rate limiting to respect API quotas
- **Debug logging** - HTTP request/response logging for troubleshooting
- **Async job handling** - Support for long-running operations like uploads and exports
- **Enterprise features** - Support for brand templates and autofill APIs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
canva-connect = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your access token
    let client = Client::new(AccessToken::new("your-access-token"));
    
    // Get user profile information
    let profile = client.user().get_profile().await?;
    println!("User: {}", profile.display_name);
    
    Ok(())
}
```

## Authentication

This library supports OAuth 2.0 authentication. You'll need to:

1. Register your application with Canva to get a client ID and secret
2. Implement the OAuth flow to get an access token
3. Use the access token to create a client

### OAuth Flow Example

```rust,skt-connect,no_run
use canva_connect::auth::{OAuthClient, OAuthConfig, Scope};
use canva_connect::{Client, auth::AccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure OAuth
    let config = OAuthConfig::new(
        "your-client-id",
        "your-client-secret", 
        "https://your-app.com/callback",
        vec![Scope::AssetRead, Scope::AssetWrite]
    );

    let oauth_client = OAuthClient::new(config);

    // Get authorization URL
    let auth_url = oauth_client.authorization_url(Some("state123"))?;
    println!("Visit: {}", auth_url);

    // After user authorizes, exchange code for token
    let token_response = oauth_client.exchange_code("authorization-code").await?;
    let access_token = AccessToken::new(token_response.access_token);

    // Create API client
    let client = Client::new(access_token);
    Ok(())
}
```

## Examples

### Upload an Asset from File

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};
use canva_connect::endpoints::assets::AssetUploadMetadata;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(AccessToken::new("your-access-token"));
    
    // Read file
    let file_data = fs::read("image.png")?;
    
    // Upload asset
    let metadata = AssetUploadMetadata {
        name: "My Image".to_string(),
        tags: vec!["rust".to_string(), "upload".to_string()],
    };
    
    let upload_job = client.assets().create_upload_job(file_data, metadata).await?;
    let result = client.assets().wait_for_upload_job(&upload_job.id).await?;
    
    println!("Uploaded asset: {}", result.asset.id);
    Ok(())
}
```

### Upload an Asset from URL

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};
use canva_connect::endpoints::assets::CreateUrlAssetUploadJobRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(AccessToken::new("your-access-token"));
    
    let request = CreateUrlAssetUploadJobRequest {
        url: "https://example.com/image.png".to_string(),
        name: "Image from URL".to_string(),
    };
    
    let upload_job = client.assets().create_url_upload_job(request).await?;
    let result = client.assets().wait_for_url_upload_job(&upload_job.id).await?;
    
    println!("Uploaded asset: {}", result.asset.id);
    Ok(())
}
```

### Get Asset Details

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(AccessToken::new("your-access-token"));
    
    // Get a specific asset by ID
    let asset = client.assets().get("asset-id").await?;
    println!("Asset: {} ({})", asset.name, asset.id);
    
    // Update asset metadata
    let update_request = canva_connect::endpoints::assets::UpdateAssetRequest {
        name: Some("Updated Asset Name".to_string()),
        tags: Some(vec!["rust".to_string(), "api".to_string()]),
    };
    
    let updated_asset = client.assets().update("asset-id", update_request).await?;
    println!("Updated asset: {}", updated_asset.name);
    
    Ok(())
}
```

### Create and Manage Folders

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};
use canva_connect::endpoints::folders::{CreateFolderRequest, UpdateFolderRequest, MoveFolderItemRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(AccessToken::new("your-access-token"));
    
    // Create a folder
    let create_request = CreateFolderRequest {
        name: "My Project".to_string(),
        parent_folder_id: "root".to_string(),
    };
    
    let folder_response = client.folders().create_folder(&create_request).await?;
    let folder = &folder_response.folder;
    println!("Created folder: {} (ID: {})", folder.name, folder.id);
    
    // List folder contents
    let list_request = canva_connect::endpoints::folders::ListFolderItemsRequest {
        limit: Some(50),
        continuation: None,
    };
    
    let items = client.folders().list_folder_items(&folder.id, &list_request).await?;
    println!("Found {} items in folder", items.items.len());
    
    Ok(())
}
```

### Export Designs

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken};
use canva_connect::endpoints::exports::{CreateExportJobRequest, ExportFormat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(AccessToken::new("your-access-token"));
    
    // Create export job
    let export_request = CreateExportJobRequest {
        design_id: "design-id".to_string(),
        format: ExportFormat::Png,
        quality: Some("high".to_string()),
    };
    
    let export_job = client.exports().create_export_job(&export_request).await?;
    println!("Created export job: {}", export_job.id);
    
    // Wait for completion
    let result = client.exports().wait_for_export_job(&export_job.id).await?;
    println!("Export completed: {}", result.export_url);
    
    Ok(())
}
```

## Running Examples

This crate includes several examples that demonstrate how to use the library.

### Setup

1. **Copy the environment template:**
   ```bash
   cp .env.example .env
   ```

2. **Configure your API credentials in `.env`:**
   ```env
   CANVA_ACCESS_TOKEN=your_access_token_here
   EXAMPLE_FILE_PATH=path/to/your/test/image.png
   ```

3. **Run the examples:**
   ```bash
   # User profile and capabilities (3 endpoints)
   cargo run --example user_profile
   
   # Asset upload from file (3 endpoints)
   cargo run --example asset_upload
   
   # Asset upload from URL (3 endpoints)
   cargo run --example url_asset_upload
   
   # Design management (3 endpoints: list, create, get)
   cargo run --example designs
   
   # Folder organization (5 endpoints: create, update, list, move)
   cargo run --example folders
   
   # Brand template operations (3 endpoints: list, get, dataset)
   cargo run --example brand_templates
   
   # Autofill brand templates with data (3 endpoints)
   cargo run --example autofill
   
   # Comment threads and replies (5 endpoints)
   cargo run --example comments
   
   # Export designs to various formats (3 endpoints)
   cargo run --example exports
   ```

### Debug Logging

All examples support debug logging to help troubleshoot API interactions:

```bash
# Enable debug logging for any example
RUST_LOG=debug cargo run --example user_profile

# This shows:
# - HTTP request details (method, URL, headers)
# - Response status codes and timing
# - Rate limiting information
# - API call flow and data
```

### Alternative: Command Line Arguments

Some examples support additional command line arguments:

```bash
# Upload asset from file (with custom file path)
cargo run --example asset_upload -- --file path/to/image.png

# Upload asset from URL (with custom URL)
cargo run --example url_asset_upload -- --url "https://example.com/image.png"
```

## API Coverage

**Complete implementation** - All 34 endpoints across 8 API modules with working examples:

### Assets API (6 endpoints + 3 upload workflows)
- ✅ `get` - Get asset details
- ✅ `update` - Update asset metadata  
- ✅ `delete` - Delete asset
- ✅ `create_upload_job` - Upload asset from file
- ✅ `get_upload_job` - Get upload job status
- ✅ `wait_for_upload_job` - Wait for upload completion
- ✅ `create_url_upload_job` - Upload asset from URL
- ✅ `get_url_upload_job` - Get URL upload job status
- ✅ `wait_for_url_upload_job` - Wait for URL upload completion

### Designs API (3 endpoints)
- ✅ `list` - List designs with filtering
- ✅ `get` - Get design details
- ✅ `create` - Create design (preset and custom dimensions)

### User API (3 endpoints)
- ✅ `get_me` - Get user identification
- ✅ `get_profile` - Get user profile
- ✅ `get_capabilities` - Get user capabilities

### Folders API (5 endpoints)
- ✅ `create_folder` - Create folder
- ✅ `get_folder` - Get folder details
- ✅ `update_folder` - Update folder
- ✅ `list_folder_items` - List folder contents
- ✅ `move_folder_item` - Move items between folders

### Brand Templates API (3 endpoints)
- ✅ `list` - List brand templates
- ✅ `get` - Get brand template details
- ✅ `get_dataset` - Get brand template datasets

### Autofill API (3 endpoints)
- ✅ `create_autofill_job` - Create autofill job
- ✅ `get_autofill_job` - Get autofill job status
- ✅ `wait_for_autofill_job` - Wait for autofill completion

### Comments API (5 endpoints)
- ✅ `create_thread` - Create comment thread
- ✅ `get_thread` - Get comment thread
- ✅ `create_reply` - Create comment reply
- ✅ `get_reply` - Get comment reply
- ✅ `list_replies` - List thread replies

### Exports API (3 endpoints)
- ✅ `create_design_export_job` - Create export job
- ✅ `get_design_export_job` - Get export job status
- ✅ `get_design_export_formats` - Get available export formats

### Total: 34 Endpoints ✅

Each endpoint has comprehensive examples demonstrating real-world usage patterns, error handling, and best practices.

## Error Handling

The library uses a comprehensive error system:


```rust

use canva_connect::{Client, auth::AccessToken, Error};
let client = Client::new(AccessToken::new("your-access-token"));
    
match client.assets().get("invalid-id").await {
        Ok(asset) => println!("Asset: {}", asset.name),
        Err(Error::Api { code, message }) => {
            println!("API error {}: {}", code, message);
        }
        Err(Error::Http(e)) => {
            println!("HTTP error: {}", e);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
    Ok(())

```

## Rate Limiting

The client includes built-in rate limiting to respect API quotas:

```rust,skt-connect,no_run
use canva_connect::{Client, auth::AccessToken, rate_limit::ApiRateLimiter};

# fn main() {
# let access_token = AccessToken::new("token");
// Create client with custom rate limiter
let rate_limiter = ApiRateLimiter::new(30); // 30 requests per minute
let client = Client::with_rate_limiter(access_token, rate_limiter);
# }
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

#### Quick Setup

For first-time setup, run the setup script:

```bash
./scripts/setup.sh
```

This will:
- Configure git settings for optimal development
- Install the pre-commit hook
- Verify Rust toolchain (rustfmt, clippy)
- Run initial checks to ensure everything works

#### Running CI Checks Locally

To avoid CI failures, run checks locally before committing:

```bash
# Run all CI checks
./scripts/check.sh

# Auto-fix formatting and clippy issues
./scripts/fix.sh
```

#### Pre-commit Hook

A pre-commit hook is automatically installed that runs all CI checks. To set it up manually:

```bash
cp scripts/check.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

#### Individual Commands

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Run integration tests (requires valid API credentials)
./scripts/integration-tests.sh
# Or manually:
CANVA_INTEGRATION_TESTS=1 cargo test --test integration

# Build release
cargo build --release
```

## License

This project is licensed under the MIT OR Apache-2.0 license.
