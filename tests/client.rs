use canva_connect::{auth::AccessToken, endpoints::assets::AssetUploadMetadata, Client, Error};

#[test]
fn test_client_creation() {
    let access_token = AccessToken::new("test_token".to_string());
    let client = Client::new(access_token);

    // Verify client was created successfully by checking we can get assets API
    let _assets_api = client.assets();
}

#[test]
fn test_client_with_different_tokens() {
    let token1 = AccessToken::new("token1".to_string());
    let token2 = AccessToken::new("token2".to_string());

    let client1 = Client::new(token1);
    let client2 = Client::new(token2);

    // Both should be created successfully
    let _assets1 = client1.assets();
    let _assets2 = client2.assets();
}

#[test]
fn test_access_token_creation() {
    let token_string = "test_access_token_12345".to_string();
    let access_token = AccessToken::new(token_string.clone());

    // Create client with this token
    let client = Client::new(access_token);
    let _assets = client.assets();
}

#[test]
fn test_client_endpoints_available() {
    let access_token = AccessToken::new("test_token".to_string());
    let client = Client::new(access_token);

    // Test that all expected endpoints are available
    let _assets = client.assets();
    let _designs = client.designs();
    let _folders = client.folders();
    let _brand_templates = client.brand_templates();
}

#[test]
fn test_error_type_properties() {
    // Test that our Error type can be created and used
    let auth_error = Error::Auth("Invalid token".to_string());
    let generic_error = Error::Generic("Something went wrong".to_string());

    // Errors should display correctly
    assert!(format!("{auth_error}").contains("Invalid token"));
    assert!(format!("{generic_error}").contains("Something went wrong"));
}

#[test]
fn test_access_token_with_empty_string() {
    let access_token = AccessToken::new("".to_string());
    let client = Client::new(access_token);

    // Should still create client (validation happens at API call time)
    let _assets = client.assets();
}

#[test]
fn test_asset_upload_metadata_creation() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["design".to_string()]);

    // Should be able to create metadata successfully
    assert!(!metadata.name_base64.is_empty());
    assert!(metadata.tags.contains(&"design".to_string()));
}
