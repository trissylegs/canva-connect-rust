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

See the [crate documentation](https://docs.rs/canva-connect) for comprehensive examples and usage patterns.

## Authentication

This library supports OAuth 2.0 authentication. You'll need to:

1. Register your application with Canva to get a client ID and secret
2. Implement the OAuth flow to get an access token
3. Use the access token to create a client

> **Note**: Complete OAuth flow examples are coming soon. For now, obtain your access token through the [Canva Developer Portal](https://www.canva.dev/docs/connect/authentication/).

## Examples

The [crate documentation](https://docs.rs/canva-connect) includes comprehensive, tested examples for:
- **Asset Management** - Upload files and URLs, get/update metadata, manage tags
- **Folder Organization** - Create folders, list contents, move items  
- **Design Export** - Export designs to various formats (PNG, PDF, etc.)
- **Error Handling** - Handle API errors, HTTP errors, and rate limiting
- **Authentication** - OAuth flows and token management
- **Rate Limiting** - Configure custom rate limits

All examples in the documentation are automatically tested to ensure they remain up-to-date and functional.

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

## Error Handling and Rate Limiting

The library includes comprehensive error handling and built-in rate limiting. See the [crate documentation](https://docs.rs/canva-connect) for detailed examples of error handling patterns and rate limiting configuration.

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

## Contributing

We welcome contributions! Please follow our pull request workflow:

1. **Find an issue** - Check our [GitHub Issues](https://github.com/canvanauts/canva-connect-rust/issues) for areas where help is needed
2. **Fork and create a branch** - Fork the repo and create a feature branch from `main`
3. **Make changes** - Implement your changes with tests and documentation
4. **Test locally** - Run `./scripts/check.sh` to ensure all CI checks pass
5. **Create a pull request** - Submit a PR targeting the `main` branch
6. **Code review** - Address any feedback from reviewers
7. **Merge** - Once approved, your PR will be merged to main

### Branch Strategy

- **main**: Production-ready code (protected, requires PR reviews)
- **feature/\***: Feature branches for individual issues
- **bugfix/\***: Bug fix branches for individual issues

### Issue Labels

We use a structured labeling system to organize issues:

- **Type**: `type:bug`, `type:feature`, `type:testing`, `type:docs`, `type:refactor`, `type:ci`, `type:security`
- **Area**: `area:assets`, `area:autofill`, `area:brand-templates`, `area:comments`, `area:designs`, `area:exports`, `area:folders`, `area:user`, `area:client`, `area:auth`, `area:errors`
- **Priority**: `priority:critical`, `priority:high`, `priority:medium`, `priority:low`
- **Status**: `status:blocked`, `status:in-progress`, `status:needs-review`, `status:good-first-issue`

Look for issues labeled `status:good-first-issue` if you're new to the project.

## License

This project is licensed under the MIT OR Apache-2.0 license.
