use canva_connect::auth::{OAuthClient, OAuthConfig, Scope, TokenStore};
use canva_connect::error::Result;
use std::env;

/// Example demonstrating OAuth 2.0 token management features
///
/// This example shows how to:
/// - Set up OAuth configuration
/// - Handle token storage and retrieval
/// - Implement automatic token refresh
/// - Use token introspection and revocation
/// - Share token stores between clients

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get OAuth configuration from environment
    let client_id = env::var("CANVA_CLIENT_ID").unwrap_or_else(|_| "your_client_id".to_string());
    let client_secret =
        env::var("CANVA_CLIENT_SECRET").unwrap_or_else(|_| "your_client_secret".to_string());
    let redirect_uri = env::var("CANVA_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:8080/callback".to_string());

    // Create OAuth configuration
    let config = OAuthConfig::new(
        client_id,
        client_secret,
        redirect_uri,
        vec![
            Scope::AssetRead,
            Scope::AssetWrite,
            Scope::DesignMetaRead,
            Scope::DesignContentRead,
        ],
    );

    println!("=== OAuth2 Token Management Demo ===");

    // Example 1: Basic OAuth Client Setup
    println!("\n1. Creating OAuth client with automatic token management...");
    let client = OAuthClient::new(config.clone());

    // Generate authorization URL (returns URL and PKCE parameters)
    let (auth_url, _pkce_params) = client.authorization_url(Some("demo_state"))?;
    println!("Authorization URL: {auth_url}");

    // Example 2: Token Store Operations
    println!("\n2. Demonstrating token store operations...");
    let token_store = TokenStore::new();

    // Initially empty
    assert!(token_store.get().await.is_none());
    println!("✓ Token store is initially empty");

    // Example 3: Manual Token Storage (in real app, this would come from OAuth flow)
    println!("\n3. Storing example tokens...");
    let example_token_set = canva_connect::auth::TokenSet {
        access_token: "example_access_token".to_string(),
        refresh_token: Some("example_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + std::time::Duration::from_secs(3600)),
        scope: Some("asset:read asset:write".to_string()),
    };

    token_store.store(example_token_set).await;
    println!("✓ Stored example token set");

    // Verify storage
    let stored_tokens = token_store.get().await.unwrap();
    println!("✓ Retrieved stored tokens: {}", stored_tokens.access_token);

    // Example 4: Token Validation and Expiry
    println!("\n4. Testing token validation and expiry...");
    let valid_token = token_store.get_valid_access_token().await;
    if valid_token.is_some() {
        println!("✓ Token is valid and not expired");
    } else {
        println!("✗ Token is expired or invalid");
    }

    // Example 5: Client with Custom Token Store
    println!("\n5. Creating client with custom token store...");
    let client_with_store = OAuthClient::with_token_store(config.clone(), token_store.clone());

    // Check if token is valid
    let is_valid = client_with_store.is_token_valid().await;
    println!("✓ Token valid in client: {is_valid}");

    // Example 6: Token Store Sharing
    println!("\n6. Demonstrating token store sharing...");
    let shared_store = TokenStore::new();
    let client1 = OAuthClient::with_token_store(config.clone(), shared_store.clone());
    let client2 = OAuthClient::with_token_store(config, shared_store);

    // Store token via client1
    let shared_token_set = canva_connect::auth::TokenSet {
        access_token: "shared_access_token".to_string(),
        refresh_token: Some("shared_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + std::time::Duration::from_secs(3600)),
        scope: Some("asset:read".to_string()),
    };

    client1.token_store().store(shared_token_set).await;

    // Both clients should see the same token
    let token1_valid = client1.is_token_valid().await;
    let token2_valid = client2.is_token_valid().await;
    println!("✓ Client1 token valid: {token1_valid}");
    println!("✓ Client2 token valid: {token2_valid}");
    assert_eq!(token1_valid, token2_valid);

    // Example 7: Token Expiry Simulation
    println!("\n7. Simulating token expiry...");
    let expired_token_set = canva_connect::auth::TokenSet {
        access_token: "expired_access_token".to_string(),
        refresh_token: Some("valid_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() - std::time::Duration::from_secs(1)),
        scope: Some("asset:read".to_string()),
    };

    let expired_client = OAuthClient::new(OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    ));

    expired_client.token_store().store(expired_token_set).await;

    // Should not be valid due to expiry
    let expired_valid = expired_client.is_token_valid().await;
    println!("✓ Expired token valid: {expired_valid}");
    assert!(!expired_valid);

    // But should have refresh token
    let has_refresh = expired_client.token_store().has_refresh_token().await;
    println!("✓ Has refresh token: {has_refresh}");

    // Example 8: Token Clearing
    println!("\n8. Testing token clearing...");
    client1.clear_tokens().await;
    let cleared_valid = client1.is_token_valid().await;
    let cleared_valid2 = client2.is_token_valid().await;
    println!("✓ Tokens cleared from shared store");
    println!("✓ Client1 valid after clear: {cleared_valid}");
    println!("✓ Client2 valid after clear: {cleared_valid2}");
    assert!(!cleared_valid && !cleared_valid2);

    println!("\n=== Demo Complete ===");
    println!("All token management operations completed successfully!");

    // Note: In a real application, you would:
    // 1. Direct users to the authorization URL
    // 2. Handle the callback with the authorization code
    // 3. Exchange the code for tokens using client.exchange_code()
    // 4. Use client.get_access_token() to get valid tokens (with auto-refresh)
    // 5. Use tokens for API calls
    // 6. Handle token introspection and revocation as needed

    Ok(())
}
