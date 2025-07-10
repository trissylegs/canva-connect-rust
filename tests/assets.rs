use canva_connect::{Client, auth::AccessToken, endpoints::assets::*};
use base64::{Engine, engine::general_purpose};

#[test]
fn test_asset_upload_metadata_creation() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["design".to_string()]);
    
    assert!(!metadata.name_base64.is_empty());
    assert!(metadata.tags.contains(&"design".to_string()));
    
    // Verify base64 encoding
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "test.png");
}

#[test]
fn test_asset_upload_metadata_with_multiple_tags() {
    let tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
    let metadata = AssetUploadMetadata::new("image.jpg", tags.clone());
    
    assert_eq!(metadata.tags, tags);
    
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "image.jpg");
}

#[test]
fn test_asset_upload_metadata_with_no_tags() {
    let metadata = AssetUploadMetadata::new("document.pdf", vec![]);
    
    assert!(metadata.tags.is_empty());
    
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "document.pdf");
}

#[test]
fn test_asset_upload_metadata_serialization() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["design".to_string()]);
    let json = serde_json::to_value(&metadata).unwrap();
    
    assert!(json.get("name_base64").is_some());
    assert!(json.get("tags").is_some());
    assert_eq!(json["name_base64"], metadata.name_base64);
}

#[test]
fn test_asset_upload_metadata_with_special_characters() {
    let filename = "file with spaces & special chars.png";
    let metadata = AssetUploadMetadata::new(filename, vec![]);
    
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), filename);
}

#[test]
fn test_assets_api_creation() {
    let access_token = AccessToken::new("test_token".to_string());
    let client = Client::new(access_token);
    
    let _assets_api = client.assets();
}

#[test]
fn test_asset_upload_metadata_serialization_structure() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["design".to_string()]);
    
    // Test that metadata has expected structure
    assert!(!metadata.name_base64.is_empty());
    assert!(metadata.tags.contains(&"design".to_string()));
    
    // Test JSON serialization
    let json = serde_json::to_value(&metadata).unwrap();
    assert!(json.get("name_base64").is_some());
    assert!(json.get("tags").is_some());
}

#[test]
fn test_asset_upload_metadata_with_unicode_filename() {
    let filename = "测试文件.png"; // Chinese characters
    let metadata = AssetUploadMetadata::new(filename, vec![]);
    
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), filename);
}

#[test]
fn test_asset_upload_metadata_debug() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["design".to_string()]);
    let debug_str = format!("{:?}", metadata);
    
    assert!(debug_str.contains("AssetUploadMetadata"));
    assert!(debug_str.contains("name_base64"));
    assert!(debug_str.contains("tags"));
}
