use canva_connect::auth::{OAuthClient, OAuthConfig, Scope};
use canva_connect::error::Error;
use std::time::Duration;

#[tokio::test]
async fn test_token_management_integration() {
    // This test would normally require real credentials and API access
    // For now, we'll test the error handling paths

    let config = OAuthConfig::new(
        "invalid_client_id",
        "invalid_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Test getting access token when no tokens are stored
    let result = client.get_access_token().await;
    assert!(matches!(result, Err(Error::Auth(_))));

    // Test token validation
    assert!(!client.is_token_valid().await);

    // Test clearing tokens when none exist
    client.clear_tokens().await;
    assert!(!client.is_token_valid().await);
}

#[tokio::test]
async fn test_token_exchange_error_handling() {
    let config = OAuthConfig::new(
        "invalid_client_id",
        "invalid_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Test token exchange with invalid code
    let result = client.exchange_code("invalid_code").await;
    assert!(matches!(result, Err(Error::Auth(_))));
}

#[tokio::test]
async fn test_refresh_token_error_handling() {
    let config = OAuthConfig::new(
        "invalid_client_id",
        "invalid_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Test refresh token when no tokens exist
    let result = client.refresh_token().await;
    assert!(matches!(result, Err(Error::Auth(_))));

    // Test refresh token when no refresh token exists
    let token_set = canva_connect::auth::TokenSet {
        access_token: "test_token".to_string(),
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };
    client.token_store().store(token_set).await;

    let result = client.refresh_token().await;
    assert!(matches!(result, Err(Error::Auth(_))));
}

#[tokio::test]
async fn test_token_introspection_error_handling() {
    let config = OAuthConfig::new(
        "invalid_client_id",
        "invalid_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Test token introspection with invalid token
    let result = client.introspect_token("invalid_token").await;
    assert!(matches!(result, Err(Error::Auth(_))));
}

#[tokio::test]
async fn test_token_revocation_error_handling() {
    let config = OAuthConfig::new(
        "invalid_client_id",
        "invalid_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Test token revocation with invalid token
    let result = client
        .revoke_token("invalid_token", Some("access_token"))
        .await;
    assert!(matches!(result, Err(Error::Auth(_))));
}

#[tokio::test]
async fn test_token_auto_refresh_scenario() {
    let config = OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Store an expired token with refresh token
    let token_set = canva_connect::auth::TokenSet {
        access_token: "expired_token".to_string(),
        refresh_token: Some("valid_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() - Duration::from_secs(1)),
        scope: None,
    };
    client.token_store().store(token_set).await;

    // Token should be invalid
    assert!(!client.is_token_valid().await);

    // Try to get access token - should attempt refresh but fail due to invalid credentials
    let result = client.get_access_token().await;
    assert!(matches!(result, Err(Error::Auth(_))));
}

#[tokio::test]
async fn test_concurrent_token_operations() {
    let config = OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Store a valid token
    let token_set = canva_connect::auth::TokenSet {
        access_token: "valid_token".to_string(),
        refresh_token: Some("refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };
    client.token_store().store(token_set).await;

    // Spawn multiple concurrent operations
    let mut handles = vec![];

    // Multiple token validation checks
    for _ in 0..10 {
        let client_clone = client.clone();
        handles.push(tokio::spawn(
            async move { client_clone.is_token_valid().await },
        ));
    }

    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result);
    }
}

// Helper function to create a client with a preloaded token for testing
#[allow(dead_code)]
async fn create_client_with_token(
    access_token: &str,
    refresh_token: Option<&str>,
    expires_in_secs: u64,
) -> OAuthClient {
    let config = OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    let token_set = canva_connect::auth::TokenSet {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.map(|t| t.to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + Duration::from_secs(expires_in_secs)),
        scope: None,
    };

    client.token_store().store(token_set).await;
    client
}

#[tokio::test]
async fn test_token_lifecycle_management() {
    let config = OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let client = OAuthClient::new(config);

    // Initially no tokens
    assert!(!client.is_token_valid().await);
    assert!(client.token_store().get().await.is_none());

    // Store a token
    let token_set = canva_connect::auth::TokenSet {
        access_token: "test_token".to_string(),
        refresh_token: Some("refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + Duration::from_secs(3600)),
        scope: Some("asset:read".to_string()),
    };
    client.token_store().store(token_set).await;

    // Token should be valid
    assert!(client.is_token_valid().await);
    let stored_token = client.token_store().get().await.unwrap();
    assert_eq!(stored_token.access_token, "test_token");
    assert_eq!(
        stored_token.refresh_token,
        Some("refresh_token".to_string())
    );

    // Clear tokens
    client.clear_tokens().await;
    assert!(!client.is_token_valid().await);
    assert!(client.token_store().get().await.is_none());
}

#[tokio::test]
async fn test_token_store_sharing() {
    let config = OAuthConfig::new(
        "test_client_id",
        "test_client_secret",
        "http://localhost:8080/callback",
        vec![Scope::AssetRead],
    );

    let token_store = canva_connect::auth::TokenStore::new();
    let client1 = OAuthClient::with_token_store(config.clone(), token_store.clone());
    let client2 = OAuthClient::with_token_store(config, token_store);

    // Store token via client1
    let token_set = canva_connect::auth::TokenSet {
        access_token: "shared_token".to_string(),
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_at: Some(std::time::Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };
    client1.token_store().store(token_set).await;

    // Both clients should see the same token
    assert!(client1.is_token_valid().await);
    assert!(client2.is_token_valid().await);

    let token1 = client1.token_store().get().await.unwrap();
    let token2 = client2.token_store().get().await.unwrap();
    assert_eq!(token1.access_token, token2.access_token);

    // Clear via client2
    client2.clear_tokens().await;

    // Both clients should see the cleared state
    assert!(!client1.is_token_valid().await);
    assert!(!client2.is_token_valid().await);
}
