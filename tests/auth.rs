use canva_connect::auth::*;

#[test]
fn test_access_token_creation() {
    let token = AccessToken::new("test_token_123".to_string());
    
    // Should create token successfully
    assert!(!token.to_string().is_empty());
}

#[test]
fn test_access_token_authorization_header() {
    let token = AccessToken::new("test_token_123".to_string());
    let auth_header = token.authorization_header();
    
    assert!(auth_header.starts_with("Bearer "));
    assert!(auth_header.contains("test_token_123"));
}

#[test]
fn test_access_token_display() {
    let token = AccessToken::new("test_token_123".to_string());
    let display_str = token.to_string();
    
    // Token should be displayed (though it may be masked)
    assert!(!display_str.is_empty());
}

#[test]
fn test_access_token_from_string() {
    let token_str = "access_token_456";
    let token = AccessToken::new(token_str.to_string());
    
    let auth_header = token.authorization_header();
    assert!(auth_header.contains(token_str));
}

#[test]
fn test_access_token_clone() {
    let token1 = AccessToken::new("test_token".to_string());
    let token2 = token1.clone();
    
    // Both tokens should have the same authorization header
    assert_eq!(token1.authorization_header(), token2.authorization_header());
}

#[test]
fn test_multiple_access_tokens() {
    let token1 = AccessToken::new("token_1".to_string());
    let token2 = AccessToken::new("token_2".to_string());
    
    // Should create different tokens
    assert_ne!(token1.authorization_header(), token2.authorization_header());
    assert!(token1.authorization_header().contains("token_1"));
    assert!(token2.authorization_header().contains("token_2"));
}

#[test]
fn test_access_token_with_special_characters() {
    let special_token = "token-with_special.chars123";
    let token = AccessToken::new(special_token.to_string());
    
    let auth_header = token.authorization_header();
    assert!(auth_header.contains(special_token));
}

#[test]
fn test_access_token_empty_string() {
    let token = AccessToken::new("".to_string());
    let auth_header = token.authorization_header();
    
    // Should still create a valid bearer header format
    assert_eq!(auth_header, "Bearer ");
}
