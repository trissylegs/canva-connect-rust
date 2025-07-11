use canva_connect::auth::{
    AccessToken, OAuthClient, OAuthConfig, Scope, TokenExchangeResponse, TokenSet, TokenStore,
};
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_token_store_basic_operations() {
    let store = TokenStore::new();

    // Initially empty
    assert!(store.get().await.is_none());
    assert!(store.get_valid_access_token().await.is_none());
    assert!(!store.has_refresh_token().await);

    // Store a token set
    let token_set = TokenSet {
        access_token: "test_access_token".to_string(),
        refresh_token: Some("test_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(Instant::now() + Duration::from_secs(3600)),
        scope: Some("asset:read asset:write".to_string()),
    };

    store.store(token_set.clone()).await;

    // Verify storage
    let stored = store.get().await.unwrap();
    assert_eq!(stored.access_token, token_set.access_token);
    assert_eq!(stored.refresh_token, token_set.refresh_token);
    assert_eq!(stored.token_type, token_set.token_type);
    assert_eq!(stored.scope, token_set.scope);

    // Verify access token retrieval
    let access_token = store.get_valid_access_token().await.unwrap();
    assert_eq!(access_token.as_str(), "test_access_token");

    // Verify refresh token check
    assert!(store.has_refresh_token().await);

    // Clear and verify
    store.clear().await;
    assert!(store.get().await.is_none());
}

#[tokio::test]
async fn test_token_set_expiry() {
    // Test non-expired token
    let mut token_set = TokenSet {
        access_token: "test_token".to_string(),
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_at: Some(Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };

    assert!(!token_set.is_expired());
    assert!(!token_set.expires_within(Duration::from_secs(1800)));
    assert!(token_set.expires_within(Duration::from_secs(7200)));

    // Test expired token
    token_set.expires_at = Some(Instant::now() - Duration::from_secs(1));
    assert!(token_set.is_expired());

    // Test token without expiry (never expires)
    token_set.expires_at = None;
    assert!(!token_set.is_expired());
    assert!(!token_set.expires_within(Duration::from_secs(3600)));
}

#[tokio::test]
async fn test_token_set_from_exchange_response() {
    let response = TokenExchangeResponse {
        access_token: "test_access_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: Some(3600),
        refresh_token: Some("test_refresh_token".to_string()),
        scope: Some("asset:read".to_string()),
    };

    let token_set = TokenSet::from_exchange_response(response);

    assert_eq!(token_set.access_token, "test_access_token");
    assert_eq!(token_set.token_type, "Bearer");
    assert_eq!(
        token_set.refresh_token,
        Some("test_refresh_token".to_string())
    );
    assert_eq!(token_set.scope, Some("asset:read".to_string()));

    // Check that expires_at is set correctly (within a reasonable range)
    let now = Instant::now();
    let expected_expiry = now + Duration::from_secs(3600);
    let actual_expiry = token_set.expires_at.unwrap();
    let diff = if actual_expiry > expected_expiry {
        actual_expiry - expected_expiry
    } else {
        expected_expiry - actual_expiry
    };
    assert!(
        diff < Duration::from_secs(1),
        "Expiry time should be within 1 second of expected"
    );
}

#[tokio::test]
async fn test_token_store_expired_token() {
    let store = TokenStore::new();

    // Store an expired token
    let token_set = TokenSet {
        access_token: "expired_token".to_string(),
        refresh_token: Some("test_refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(Instant::now() - Duration::from_secs(1)),
        scope: None,
    };

    store.store(token_set).await;

    // Should not return expired token as valid
    assert!(store.get_valid_access_token().await.is_none());

    // But should still have refresh token
    assert!(store.has_refresh_token().await);

    // Raw get should still return the token set
    assert!(store.get().await.is_some());
}

#[tokio::test]
async fn test_access_token_creation() {
    let token = AccessToken::new("test_token");
    assert_eq!(token.as_str(), "test_token");
    assert_eq!(token.authorization_header(), "Bearer test_token");
    assert_eq!(token.to_string(), "Bearer test_token");

    // Test From implementations
    let token_from_string = AccessToken::from("test_token".to_string());
    assert_eq!(token_from_string.as_str(), "test_token");

    let token_from_str = AccessToken::from("test_token");
    assert_eq!(token_from_str.as_str(), "test_token");
}

#[tokio::test]
async fn test_oauth_client_creation() {
    let config = OAuthConfig::new(
        "client_id",
        "client_secret",
        "http://127.0.0.1:8080/callback",
        vec![Scope::AssetRead, Scope::AssetWrite],
    );

    let client = OAuthClient::new(config);

    // Test that client is created with token store
    assert!(!client.is_token_valid().await);
    assert!(client.token_store().get().await.is_none());
}

#[tokio::test]
async fn test_oauth_client_with_custom_token_store() {
    let config = OAuthConfig::new(
        "client_id",
        "client_secret",
        "http://127.0.0.1:8080/callback",
        vec![Scope::AssetRead],
    );

    let token_store = TokenStore::new();
    let token_set = TokenSet {
        access_token: "custom_token".to_string(),
        refresh_token: None,
        token_type: "Bearer".to_string(),
        expires_at: Some(Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };
    token_store.store(token_set).await;

    let client = OAuthClient::with_token_store(config, token_store);

    // Should have the pre-stored token
    assert!(client.is_token_valid().await);
    let stored_token = client.token_store().get().await.unwrap();
    assert_eq!(stored_token.access_token, "custom_token");
}

#[tokio::test]
async fn test_token_store_thread_safety() {
    let store = TokenStore::new();
    let _store_clone = store.clone();

    // Spawn multiple tasks to test thread safety
    let mut handles = vec![];

    // Task 1: Store tokens
    let store1 = store.clone();
    handles.push(tokio::spawn(async move {
        let token_set = TokenSet {
            access_token: "token1".to_string(),
            refresh_token: None,
            token_type: "Bearer".to_string(),
            expires_at: Some(Instant::now() + Duration::from_secs(3600)),
            scope: None,
        };
        store1.store(token_set).await;
    }));

    // Task 2: Read tokens
    let store2 = store.clone();
    handles.push(tokio::spawn(async move {
        for _ in 0..10 {
            let _ = store2.get().await;
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }));

    // Task 3: Check valid tokens
    let store3 = store.clone();
    handles.push(tokio::spawn(async move {
        for _ in 0..10 {
            let _ = store3.get_valid_access_token().await;
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }));

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify final state
    let final_token = store.get().await;
    assert!(final_token.is_some());
    assert_eq!(final_token.unwrap().access_token, "token1");
}

#[tokio::test]
async fn test_token_clear_functionality() {
    let store = TokenStore::new();

    // Store a token with refresh token
    let token_set = TokenSet {
        access_token: "test_token".to_string(),
        refresh_token: Some("refresh_token".to_string()),
        token_type: "Bearer".to_string(),
        expires_at: Some(Instant::now() + Duration::from_secs(3600)),
        scope: None,
    };

    store.store(token_set).await;

    // Verify storage
    assert!(store.get().await.is_some());
    assert!(store.get_valid_access_token().await.is_some());
    assert!(store.has_refresh_token().await);

    // Clear and verify
    store.clear().await;
    assert!(store.get().await.is_none());
    assert!(store.get_valid_access_token().await.is_none());
    assert!(!store.has_refresh_token().await);
}

#[cfg(test)]
mod oauth_client_tests {
    use super::*;

    fn create_test_config() -> OAuthConfig {
        OAuthConfig::new(
            "test_client_id",
            "test_client_secret",
            "http://127.0.0.1:8080/callback",
            vec![Scope::AssetRead, Scope::AssetWrite],
        )
    }

    #[tokio::test]
    async fn test_authorization_url_generation() {
        let config = create_test_config();
        let client = OAuthClient::new(config);

        let (url, _pkce) = client.authorization_url(Some("test_state")).unwrap();

        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A8080%2Fcallback"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("scope=asset%3Aread+asset%3Awrite"));
        assert!(url.contains("state=test_state"));
        assert!(url.contains("code_challenge"));
        assert!(url.contains("code_challenge_method=S256"));
    }

    #[tokio::test]
    async fn test_clear_tokens() {
        let config = create_test_config();
        let client = OAuthClient::new(config);

        // Manually store a token
        let token_set = TokenSet {
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh_token".to_string()),
            token_type: "Bearer".to_string(),
            expires_at: Some(Instant::now() + Duration::from_secs(3600)),
            scope: None,
        };
        client.token_store().store(token_set).await;

        // Verify token is stored
        assert!(client.is_token_valid().await);

        // Clear and verify
        client.clear_tokens().await;
        assert!(!client.is_token_valid().await);
        assert!(client.token_store().get().await.is_none());
    }
}
