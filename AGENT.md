# AGENT.md - Canva Connect Rust Client

## Project Structure
- `public-api.json` - OpenAPI 3.0 specification for Canva Connect API (JSON format for easier searching with `jq`)
- `src/` - Rust client library source code
- `examples/` - Usage examples
- `scripts/` - Development scripts
- `tests/` - Unit and integration tests
- `Cargo.toml` - Rust project configuration

## Build/Test Commands
- `cargo check` - Check code for compilation errors
- `cargo build` - Build the library
- `cargo test` - Run unit tests
- `cargo doc` - Generate documentation
- `cp .env.example .env` - Set up environment configuration
- `cargo run --example asset_upload` - Run asset upload example (needs .env setup)
- `cargo run --example url_asset_upload` - Run URL asset upload example (needs .env setup)
- `cargo run --example user_profile` - Run user profile example (needs .env setup)
- `cargo run --example designs` - Run design API example (needs .env setup)
- `cargo run --example folders` - Run folder organization example (needs .env setup)
- `cargo run --example observability --features observability` - Run observability example with tracing
- `cargo run --example asset_upload -- --file path/to/file` - Run with custom file path
- `cargo run --example url_asset_upload -- --url "https://rustacean.net/assets/rustacean-flat-happy.png"` - Run with custom URL

**Available examples for all APIs:**
- `asset_upload` - Upload files as assets
- `url_asset_upload` - Upload assets from URLs  
- `designs` - Create and manage designs
- `folders` - Create and organize content in folders
- `user_profile` - Get user information and capabilities
- `observability` - Demonstrate tracing integration (requires `observability` feature)

## Documentation Testing
- `cargo test --test skeptic` - Test README.md code examples with skeptic (currently has dependency resolution issues)
- Skeptic is configured to test all code examples in README.md for compilation
- Examples marked with `no_run` compile but don't execute (to avoid needing API tokens)

## Integration Tests
Integration tests make real API calls to Canva Connect and require valid credentials:

- `./scripts/integration-tests.sh` - Run all integration tests (loads .env automatically)
- `./scripts/integration-tests.sh test_name` - Run specific integration test
- `CANVA_INTEGRATION_TESTS=1 cargo test --test integration` - Manual method
- `CANVA_INTEGRATION_TESTS=1 cargo test` - Run all tests including integration tests

**Setup for integration tests:**
1. Copy `.env.example` to `.env`: `cp .env.example .env`
2. Add your Canva Connect API access token to `.env`: `CANVA_ACCESS_TOKEN=your-token`
3. Run integration tests: `./scripts/integration-tests.sh`
4. Tests will automatically clean up any assets they create
5. Tests respect rate limits with built-in delays

**Available integration tests:**
- `test_get_user_profile` - Test user profile retrieval
- `test_get_user_capabilities` - Test user capabilities checking
- `test_url_asset_upload_workflow` - Test complete asset upload workflow
- `test_asset_error_handling` - Test asset API error handling
- `test_list_designs` - Test design listing with filters
- `test_create_and_get_design` - Test design creation and retrieval (⚠️ creates designs that cannot be deleted)
- `test_create_custom_design` - Test custom dimension design creation (⚠️ creates designs that cannot be deleted)
- `test_design_error_handling` - Test design API error handling

**Note**: The Canva Connect API does not provide an endpoint to delete designs, so integration tests that create designs will leave them in the user's account. Asset tests automatically clean up created assets.

## Development Scripts
- `./scripts/setup.sh` - One-time setup script for development environment
- `./scripts/check.sh` - Run all CI checks locally (formatting, clippy, tests, build)
- `./scripts/fix.sh` - Auto-fix formatting and clippy issues
- `./scripts/docs.sh` - Generate Rust documentation (use `--open` to open in browser)
- `./scripts/integration-tests.sh` - Run integration tests with automatic .env loading
- `./scripts/pre-commit.sh` - Pre-commit hook logic (version controlled)
- **Pre-commit hook**: Automatically runs all CI checks before each commit

## Quick Setup
For new developers:
```bash
./scripts/setup.sh  # One-time setup
```

## Local CI Checks
Always run before committing to avoid CI failures:
```bash
./scripts/check.sh  # Run all checks
./scripts/fix.sh    # Auto-fix issues
```

Individual commands:
- `cargo fmt --all` - Format code
- `cargo clippy --all-targets --all-features -- -D warnings` - Lint code
- `cargo test --all-features` - Run all tests
- `cargo build --release` - Build optimized version

## Codebase Structure
- `src/lib.rs` - Main library entry point
- `src/client.rs` - HTTP client implementation with rate limiting
- `src/auth.rs` - OAuth 2.0 authentication flow
- `src/error.rs` - Comprehensive error handling
- `src/models.rs` - Data structures for API responses
- `src/rate_limit.rs` - Rate limiting implementation
- `src/endpoints/` - API endpoint implementations
  - `assets.rs` - Assets API (upload, manage, retrieve)
  - `autofill.rs` - Autofill API (brand template data autofill)
  - `brand_templates.rs` - Brand Templates API (templates and datasets)
  - `comments.rs` - Comments API (threads and replies)
  - `designs.rs` - Designs API (create, list, get)
  - `exports.rs` - Exports API (export to various formats)
  - `folders.rs` - Folders API (organize content)
  - `user.rs` - User API (profile, capabilities, identification)
  - `mod.rs` - Module organization and endpoint exports

## Code Style
- Use `async/await` for all API calls
- Return `Result<T, Error>` for fallible operations
- Use `serde` for JSON serialization/deserialization
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `#[derive(Debug, Clone)]` for most data structures
- Implement comprehensive error handling with custom error types

## Security Guidelines
- **Access tokens should never be passed via command line arguments** - they are visible in process lists, shell history, and logs. Always use .env files for credentials.

## API Design Patterns
- **Tagged Unions (oneOf)**: The Canva API uses oneOf patterns with discriminator fields. In Rust, model these as enums with `#[serde(tag = "type", rename_all = "snake_case")]` instead of separate structs with explicit type fields. For example, `DesignTypeInput` is a tagged union with `Preset` and `Custom` variants where serde automatically handles the `type` discriminator field.

## OAuth Scopes Required by APIs
Different APIs require different OAuth scopes. Here's a quick reference:

- **Assets API**: `asset:read`, `asset:write`
- **Autofill API**: `brandtemplate:content:read` (for autofill operations)
- **Brand Templates API**: `brandtemplate:meta:read`, `brandtemplate:content:read`
- **Comments API**: `comment:read`, `comment:write`
- **Designs API**: `design:meta:read`, `design:content:read`, `design:content:write`
- **Exports API**: `design:content:read`
- **Folders API**: `folder:read`, `folder:write`
- **User API**: No specific scopes required (basic user info always accessible)

## API Specification
The `public-api.json` file contains the complete OpenAPI 3.0 specification for the Canva Connect API. You can search it efficiently using `jq`:

```bash
# Find all endpoint paths
jq '.paths | keys[]' public-api.json

# Find design-related endpoints
jq '.paths | keys[] | select(contains("design"))' public-api.json

# Get schema for a specific model
jq '.components.schemas.Asset' public-api.json

# Find all schemas containing "design"
jq '.components.schemas | keys[] | select(contains("Design"))' public-api.json
```

## API Implementation Status
- ✅ Core client with OAuth 2.0 authentication
- ✅ Rate limiting and error handling
- ✅ Assets API (upload, get, update, delete)
- ✅ Autofill API (create autofill jobs, get job status)
- ✅ Brand Templates API (list templates, get template details, get datasets)
- ✅ Comments API (create/get threads, create/get/list replies)
- ✅ Designs API (list, get, create)
- ✅ Exports API (create export jobs, get job status, get export formats)
- ✅ Folders API (create/get/update folders, list items, move items)
- ✅ User API (profile, capabilities, identification)
- ✅ Observability (OpenTelemetry tracing with feature flag)
- ✅ Integration tests with automatic cleanup and rate limiting
- ✅ **Complete API coverage** - All major Canva Connect endpoints implemented

## Dependencies
- `reqwest` - HTTP client with async support
- `serde` - JSON serialization
- `tokio` - Async runtime
- `governor` - Rate limiting
- `thiserror` - Error handling macros
- `opentelemetry` - Distributed tracing (optional, with `observability` feature)
- `tracing` - Application-level tracing (optional, with `observability` feature)
