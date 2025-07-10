use canva_connect::{Client, auth::AccessToken, models::*, endpoints::assets::AssetUploadMetadata};
use serde_json::json;
use base64::{Engine, engine::general_purpose};

#[test]
fn test_job_status_serialization() {
    let status = JobStatus::InProgress;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""in_progress""#);
    
    let status = JobStatus::Success;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""success""#);
    
    let status = JobStatus::Failed;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""failed""#);
}

#[test]
fn test_job_status_deserialization() {
    let json = r#""in_progress""#;
    let status: JobStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status, JobStatus::InProgress);
    
    let json = r#""success""#;
    let status: JobStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status, JobStatus::Success);
    
    let json = r#""failed""#;
    let status: JobStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status, JobStatus::Failed);
}

#[test]
fn test_asset_upload_metadata_new() {
    let metadata = AssetUploadMetadata::new("test_file.png", vec![]);
    
    // Verify name_base64 is correctly base64 encoded
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "test_file.png");
}

#[test]
fn test_asset_upload_metadata_serialization() {
    let metadata = AssetUploadMetadata::new("test_file.png", vec!["tag1".to_string()]);
    let json = serde_json::to_value(&metadata).unwrap();
    
    assert!(json.get("name_base64").is_some());
    assert_eq!(json.get("name_base64").unwrap().as_str().unwrap(), metadata.name_base64);
}

#[test]
fn test_client_creation() {
    let access_token = AccessToken::new("test_token".to_string());
    let client = Client::new(access_token);
    
    // Verify client was created successfully by checking we can get assets API
    let _assets_api = client.assets();
}

#[test]
fn test_asset_deserialization() {
    let json = json!({
        "id": "asset_123",
        "name": "test_asset",
        "tags": [],
        "type": "image",
        "thumbnail": {
            "url": "https://example.com/thumb.jpg",
            "width": 150,
            "height": 150
        },
        "created_at": 1640995200,
        "updated_at": 1640995200
    });
    
    let asset: Asset = serde_json::from_value(json).unwrap();
    assert_eq!(asset.id, "asset_123");
    assert_eq!(asset.name, "test_asset");
    assert!(asset.thumbnail.is_some());
    if let Some(thumbnail) = asset.thumbnail {
        assert_eq!(thumbnail.url, "https://example.com/thumb.jpg");
        assert_eq!(thumbnail.width, 150);
        assert_eq!(thumbnail.height, 150);
    }
}

#[test]
fn test_thumbnail_deserialization() {
    let json = json!({
        "url": "https://example.com/thumb.jpg",
        "width": 200,
        "height": 150
    });
    
    let thumbnail: Thumbnail = serde_json::from_value(json).unwrap();
    assert_eq!(thumbnail.url, "https://example.com/thumb.jpg");
    assert_eq!(thumbnail.width, 200);
    assert_eq!(thumbnail.height, 150);
}

#[test]
fn test_unix_timestamp_serialization() {
    let timestamp = chrono::DateTime::from_timestamp(1640995200, 0).unwrap();
    let json = serde_json::to_value(&timestamp).unwrap();
    // This will be the full RFC3339 format, not just a timestamp
    assert!(json.is_string());
}

#[test]
fn test_asset_upload_job_deserialization() {
    let json = json!({
        "id": "test_job_id",
        "status": "in_progress",
        "asset": null,
        "error": null
    });
    
    let job: AssetUploadJob = serde_json::from_value(json).unwrap();
    assert_eq!(job.id, "test_job_id");
    assert_eq!(job.status, JobStatus::InProgress);
    assert!(job.asset.is_none());
}

#[test]
fn test_asset_upload_metadata_with_special_chars() {
    let metadata = AssetUploadMetadata::new("file with spaces & special.png", vec![]);
    
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "file with spaces & special.png");
}

#[test] 
fn test_api_error_code_display() {
    use canva_connect::error::ApiErrorCode;
    
    assert_eq!(format!("{}", ApiErrorCode::NotFound), "NOT_FOUND");
    assert_eq!(format!("{}", ApiErrorCode::Unauthorized), "UNAUTHORIZED");
    assert_eq!(format!("{}", ApiErrorCode::Unknown("CUSTOM".to_string())), "CUSTOM");
}

#[test]
fn test_api_error_code_from_string() {
    use canva_connect::error::ApiErrorCode;
    
    assert_eq!(ApiErrorCode::from("NOT_FOUND".to_string()), ApiErrorCode::NotFound);
    assert_eq!(ApiErrorCode::from("UNAUTHORIZED".to_string()), ApiErrorCode::Unauthorized);
    assert_eq!(ApiErrorCode::from("UNKNOWN_CODE".to_string()), ApiErrorCode::Unknown("UNKNOWN_CODE".to_string()));
}
