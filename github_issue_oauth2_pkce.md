# Implement Full OAuth2 Flow Module with PKCE Support

## Summary
The current auth implementation in `src/auth.rs` provides basic OAuth2 structures but lacks a complete implementation of the OAuth2 Authorization Code flow with PKCE (Proof Key for Code Exchange) as required by the Canva Connect API. We need to implement a full OAuth2 flow module that handles the complete authorization process.

## Background
The Canva Connect API requires OAuth 2.0 with the Authorization Code flow with PKCE using SHA-256, as documented in the Connect API authentication documentation. The current implementation has:

✅ Basic structures (`AccessToken`, `Scope`, `OAuthConfig`, `OAuthClient`)  
✅ Authorization URL generation  
✅ Token exchange functionality  
❌ **Missing PKCE implementation**  
❌ **Missing complete OAuth2 flow orchestration**  
❌ **Missing state parameter handling for CSRF protection**  
❌ **Missing refresh token management**

## Requirements

Based on [Canva Connect API Authentication Documentation](https://www.canva.dev/docs/connect/authentication/), the OAuth2 flow must implement:

### Core PKCE Requirements
1. **Code Verifier Generation**: High-entropy cryptographically random string (43-128 chars, ASCII letters/numbers/`-._~`)
2. **Code Challenge Generation**: SHA-256 hash of code verifier, base64url encoded  
3. **Code Challenge Method**: Must be `S256`
4. **Secure Storage**: Code verifier must be securely stored and not accessible to user/browser

### OAuth2 Flow Requirements
1. **Authorization URL Generation** with PKCE parameters:
   - `code_challenge` (derived from code_verifier)
   - `code_challenge_method=S256`
   - `scope` (space-separated list)
   - `response_type=code`
   - `client_id`
   - `state` (optional but recommended for CSRF protection)
   - `redirect_uri` (optional if only one configured)

2. **Authorization Code Exchange**:
   - Exchange authorization code for access token
   - Include `code_verifier` in token request
   - Handle both access and refresh tokens
   - Proper error handling

3. **Token Management**:
   - Access token storage and retrieval
   - Refresh token handling
   - Token expiry management
   - Token introspection
   - Token revocation

4. **Security Features**:
   - State parameter generation and validation (CSRF protection)
   - Secure random string generation
   - Proper error handling for security violations

## Proposed Implementation

### New Modules Structure
```
src/auth/
├── mod.rs           # Main auth module exports
├── scopes.rs        # OAuth scopes (existing)
├── pkce.rs          # PKCE implementation
├── flow.rs          # Complete OAuth2 flow orchestration  
├── tokens.rs        # Token management (access, refresh)
├── security.rs      # Security utilities (state, random generation)
└── errors.rs        # Auth-specific errors
```

### Key Components

#### 1. PKCE Implementation (`src/auth/pkce.rs`)
```rust
pub struct PkceChallenge {
    pub code_verifier: String,
    pub code_challenge: String,
    pub code_challenge_method: String, // Always "S256"
}

impl PkceChallenge {
    pub fn new() -> Result<Self, Error>;
    pub fn verify(&self, verifier: &str) -> bool;
}
```

#### 2. OAuth2 Flow Manager (`src/auth/flow.rs`)
```rust
pub struct OAuth2Flow {
    config: OAuthConfig,
    client: reqwest::Client,
}

impl OAuth2Flow {
    pub async fn start_authorization(&self, scopes: &[Scope], state: Option<String>) -> Result<AuthorizationSession, Error>;
    pub async fn complete_authorization(&self, session: AuthorizationSession, auth_code: &str, received_state: Option<&str>) -> Result<TokenResponse, Error>;
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse, Error>;
    pub async fn introspect_token(&self, token: &str) -> Result<TokenIntrospection, Error>;
    pub async fn revoke_token(&self, token: &str) -> Result<(), Error>;
}

pub struct AuthorizationSession {
    pub authorization_url: String,
    pub state: Option<String>,
    pkce_challenge: PkceChallenge,
}
```

#### 3. Enhanced Token Management (`src/auth/tokens.rs`)
```rust
pub struct TokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scope: Option<String>,
    pub token_type: String,
}

pub struct TokenManager {
    // Token storage and management logic
    pub async fn store_tokens(&mut self, tokens: TokenResponse) -> Result<(), Error>;
    pub async fn get_valid_access_token(&mut self) -> Result<AccessToken, Error>;
    pub async fn refresh_if_needed(&mut self) -> Result<(), Error>;
}
```

## Acceptance Criteria

- [ ] **PKCE Implementation**: Complete PKCE challenge/verifier generation and validation
- [ ] **Authorization URL**: Generate complete authorization URLs with all required PKCE parameters  
- [ ] **State Handling**: Generate and validate state parameters for CSRF protection
- [ ] **Token Exchange**: Exchange authorization codes for tokens using PKCE verifier
- [ ] **Token Management**: Handle access tokens, refresh tokens, expiry, and renewal
- [ ] **Security**: Implement secure random generation for code verifiers and state
- [ ] **Error Handling**: Comprehensive error handling for all OAuth2 scenarios
- [ ] **Documentation**: Complete rustdoc documentation with examples
- [ ] **Examples**: Working examples demonstrating the full OAuth2 flow
- [ ] **Tests**: Unit tests for all components and integration tests for complete flow

## Implementation Tasks

### Phase 1: Core PKCE Implementation
- [ ] Implement `PkceChallenge` struct with secure random generation
- [ ] Add cryptographic utilities for SHA-256 and base64url encoding
- [ ] Add comprehensive tests for PKCE generation and validation

### Phase 2: OAuth2 Flow Orchestration  
- [ ] Implement `OAuth2Flow` manager for complete flow coordination
- [ ] Add `AuthorizationSession` for managing authorization state
- [ ] Implement state parameter generation and validation
- [ ] Add authorization URL generation with PKCE parameters

### Phase 3: Token Management
- [ ] Enhance token exchange to include PKCE verifier
- [ ] Implement refresh token handling
- [ ] Add token introspection and revocation
- [ ] Create `TokenManager` for automatic token renewal

### Phase 4: Examples and Documentation
- [ ] Create comprehensive OAuth2 flow example
- [ ] Add documentation with step-by-step usage guide
- [ ] Update existing examples to use new OAuth2 flow
- [ ] Add integration tests with real API calls

## Security Considerations

1. **Code Verifier Security**: Must be cryptographically secure and never exposed to client
2. **State Validation**: Always validate state parameter to prevent CSRF attacks  
3. **Token Storage**: Secure storage of refresh tokens and sensitive data
4. **Error Handling**: Avoid leaking sensitive information in error messages
5. **Timing Attacks**: Use constant-time comparison for sensitive operations

## Dependencies

Potential new dependencies needed:
- `sha2` - For SHA-256 hashing (or use existing crypto crate)
- `base64` - For base64url encoding (or extend existing usage)
- `rand` - For cryptographically secure random generation (likely already available)

## References

- [Canva Connect API Authentication Documentation](https://www.canva.dev/docs/connect/authentication/)
- [RFC 7636: PKCE Specification](https://datatracker.ietf.org/doc/html/rfc7636)
- [RFC 6749: OAuth 2.0 Authorization Framework](https://datatracker.ietf.org/doc/html/rfc6749)
- [OAuth Community Libraries](https://oauth.net/code/)

## Related Issues

This implementation should provide the foundation for:
- Enhanced security for all API operations
- Better developer experience with complete OAuth2 flow
- Compliance with OAuth2 security best practices
- Foundation for advanced features like token refresh automation

---

**Priority**: High - Required for production-ready OAuth2 compliance  
**Complexity**: Medium-High - Involves cryptographic operations and security considerations  
**Type**: Feature - New OAuth2 flow module implementation