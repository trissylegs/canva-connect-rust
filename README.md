# Canva Connect Rust Client

A Rust client library for the [Canva Connect API](https://www.canva.dev/docs/connect/) that provides a safe and ergonomic interface for interacting with Canva's design platform.

## Features

- **Async/await support** - Built on `tokio` and `reqwest`
- **Type safety** - Strongly typed API with comprehensive error handling
- **OAuth 2.0 authentication** - Full support for Canva's OAuth flow
- **Rate limiting** - Built-in rate limiting to respect API quotas
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

```rust,no_run
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

```rust,no_run
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

```rust,no_run
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

```rust,no_run
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

```rust,no_run
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
   # User profile and capabilities
   cargo run --example user_profile
   
   # Asset upload from file
   cargo run --example asset_upload
   
   # Asset upload from URL
   cargo run --example url_asset_upload
   
   # Design management (list, create, get)
   cargo run --example designs
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

Currently implemented endpoints:

### Assets
- ✅ List assets
- ✅ Get asset details
- ✅ Update asset
- ✅ Delete asset
- ✅ Upload asset (file)
- ✅ Upload asset (URL)
- ✅ Get upload job status

### Designs
- ✅ List designs
- ✅ Get design details
- ✅ Create design (preset and custom)

### User
- ✅ Get user profile
- ✅ Get user capabilities
- ✅ Get user identification

### Coming Soon
- Folders API
- Brand Templates API
- Autofill API
- Comments API
- Exports API

## Error Handling

The library uses a comprehensive error system:

```rust,no_run
use canva_connect::{Client, auth::AccessToken, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
}
```

## Rate Limiting

The client includes built-in rate limiting to respect API quotas:

```rust,no_run
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
