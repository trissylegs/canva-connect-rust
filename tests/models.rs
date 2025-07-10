use canva_connect::{models::*, endpoints::assets::AssetUploadMetadata};
use serde_json::json;
use chrono::{DateTime, Utc};
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
    let metadata = AssetUploadMetadata::new("test_file.png", vec![]);
    let json = serde_json::to_value(&metadata).unwrap();
    
    assert!(json.get("name_base64").is_some());
    assert_eq!(json.get("name_base64").unwrap().as_str().unwrap(), metadata.name_base64);
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
    assert!(job.error.is_none());
}

#[test]
fn test_asset_upload_job_with_asset() {
    let json = json!({
        "id": "test_job_id", 
        "status": "success",
        "asset": {
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
        },
        "error": null
    });
    
    let job: AssetUploadJob = serde_json::from_value(json).unwrap();
    assert_eq!(job.id, "test_job_id");
    assert_eq!(job.status, JobStatus::Success);
    assert!(job.asset.is_some());
    
    let asset = job.asset.unwrap();
    assert_eq!(asset.id, "asset_123");
    assert_eq!(asset.name, "test_asset");
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
        "width": 150,
        "height": 150
    });
    
    let thumbnail: Thumbnail = serde_json::from_value(json).unwrap();
    assert_eq!(thumbnail.url, "https://example.com/thumb.jpg");
    assert_eq!(thumbnail.width, 150);
    assert_eq!(thumbnail.height, 150);
}

#[test]
fn test_job_generic_structure() {
    // Test basic Job structure properties
    let job: Job<String> = Job {
        id: "job_123".to_string(),
        status: JobStatus::Success,
        result: Some("success_data".to_string()),
        error: None,
    };
    
    assert_eq!(job.id, "job_123");
    assert_eq!(job.status, JobStatus::Success);
    assert!(job.result.is_some());
    assert!(job.error.is_none());
    assert_eq!(job.result.unwrap(), "success_data");
}

#[test]
fn test_asset_upload_metadata_with_tags() {
    let metadata = AssetUploadMetadata::new("test.png", vec!["tag1".to_string(), "tag2".to_string()]);
    
    let json = serde_json::to_value(&metadata).unwrap();
    assert!(json.get("name_base64").is_some());
    assert!(json.get("tags").is_some());
    
    // Verify the decoded name
    let decoded = general_purpose::STANDARD.decode(&metadata.name_base64).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "test.png");
}

#[test]
fn test_asset_with_optional_thumbnail() {
    let json = json!({
        "id": "asset_123",
        "name": "test_asset",
        "tags": ["tag1", "tag2"],
        "type": "image",
        "thumbnail": null,
        "created_at": 1640995200,
        "updated_at": 1640995200
    });
    
    let asset: Asset = serde_json::from_value(json).unwrap();
    assert_eq!(asset.id, "asset_123");
    assert_eq!(asset.name, "test_asset");
    assert!(asset.thumbnail.is_none());
}

#[test]
fn test_datetime_serialization() {
    let timestamp = DateTime::from_timestamp(1640995200, 0).unwrap();
    let json = serde_json::to_value(&timestamp).unwrap();
    // chrono serializes to RFC3339 format by default
    assert!(json.is_string());
    assert!(json.as_str().unwrap().contains("2022-01-01"));
}

#[test]
fn test_datetime_deserialization() {
    let json = json!("2022-01-01T00:00:00Z");
    let timestamp: DateTime<Utc> = serde_json::from_value(json).unwrap();
    assert_eq!(timestamp, DateTime::from_timestamp(1640995200, 0).unwrap());
}
