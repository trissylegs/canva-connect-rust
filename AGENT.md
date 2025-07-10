# AGENT.md - Canva Connect Rust Client

## Project Structure
- `public-api.yml` - OpenAPI 3.0 specification for Canva Connect API
- `src/` - Rust client library source code
- `examples/` - Usage examples
- `scripts/` - Development scripts
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
- `cargo run --example observability --features observability` - Run observability example with tracing
- `cargo run --example asset_upload -- --token TOKEN --file path/to/file` - Run with CLI args
- `cargo run --example url_asset_upload -- --url "https://rustacean.net/assets/rustacean-flat-happy.png"` - Run with CLI args
- `cargo run --example user_profile -- --token TOKEN` - Run with CLI args

## Integration Tests
Integration tests make real API calls to Canva Connect and require valid credentials:

- `CANVA_INTEGRATION_TESTS=1 cargo test --test integration` - Run integration tests only
- `CANVA_INTEGRATION_TESTS=1 cargo test` - Run all tests including integration tests

**Setup for integration tests:**
1. Set `CANVA_ACCESS_TOKEN` environment variable with a valid token
2. Set `CANVA_INTEGRATION_TESTS=1` to enable the tests
3. Tests will automatically clean up any assets they create
4. Tests respect rate limits with built-in delays

## Development Scripts
- `./scripts/setup.sh` - One-time setup script for development environment
- `./scripts/check.sh` - Run all CI checks locally (formatting, clippy, tests, build)
- `./scripts/fix.sh` - Auto-fix formatting and clippy issues
- `./scripts/docs.sh` - Generate Rust documentation (use `--open` to open in browser)
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
  - `assets.rs` - Assets API (fully implemented)
  - `mod.rs` - Other endpoints (stub implementations)

## Code Style
- Use `async/await` for all API calls
- Return `Result<T, Error>` for fallible operations
- Use `serde` for JSON serialization/deserialization
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `#[derive(Debug, Clone)]` for most data structures
- Implement comprehensive error handling with custom error types

## API Implementation Status
- ✅ Core client with OAuth 2.0 authentication
- ✅ Rate limiting and error handling
- ✅ Assets API (upload, get, update, delete)
- ✅ User API (profile, capabilities, identification)
- ✅ Observability (OpenTelemetry tracing with feature flag)
- 🚧 Other endpoints (stubs created, need implementation)

## Dependencies
- `reqwest` - HTTP client with async support
- `serde` - JSON serialization
- `tokio` - Async runtime
- `governor` - Rate limiting
- `thiserror` - Error handling macros
- `opentelemetry` - Distributed tracing (optional, with `observability` feature)
- `tracing` - Application-level tracing (optional, with `observability` feature)
