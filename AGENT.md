# AGENT.md - Canva Connect Rust Client

## Project Structure
- `public-api.yml` - OpenAPI 3.0 specification for Canva Connect API
- `src/` - Rust client library source code
- `examples/` - Usage examples
- `Cargo.toml` - Rust project configuration

## Build/Test Commands
- `cargo check` - Check code for compilation errors
- `cargo build` - Build the library
- `cargo test` - Run unit tests
- `cargo doc` - Generate documentation
- `cp .env.example .env` - Set up environment configuration
- `cargo run --example basic_usage` - Run basic usage example (needs .env setup)
- `cargo run --example asset_upload` - Run asset upload example (needs .env setup)
- `cargo run --example basic_usage -- --token TOKEN` - Run with CLI args instead of .env
- `cargo run --example asset_upload -- --token TOKEN --file path/to/file` - Run with CLI args

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
- ✅ Assets API (upload, list, get, update, delete)
- 🚧 Other endpoints (stubs created, need implementation)

## Dependencies
- `reqwest` - HTTP client with async support
- `serde` - JSON serialization
- `tokio` - Async runtime
- `governor` - Rate limiting
- `thiserror` - Error handling macros
