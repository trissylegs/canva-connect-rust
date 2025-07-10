//! Data models for the Canva Connect API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Common pagination response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// List of items
    pub items: Vec<T>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// Whether there are more items
    pub has_more: bool,
}

/// Asset metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Asset ID
    pub id: String,
    /// Asset name
    pub name: String,
    /// Asset tags
    pub tags: Vec<String>,
    /// Asset type
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    /// Asset thumbnail URL
    pub thumbnail: Option<Thumbnail>,
    /// Asset creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Asset last updated timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Asset type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    /// Image asset
    Image,
    /// Video asset
    Video,
    /// Audio asset
    Audio,
}

/// Thumbnail information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    /// Thumbnail URL
    pub url: String,
    /// Thumbnail width
    pub width: u32,
    /// Thumbnail height
    pub height: u32,
}

/// Design metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Design {
    /// Design ID
    pub id: String,
    /// Design title
    pub title: String,
    /// Design type
    #[serde(rename = "type")]
    pub design_type: DesignType,
    /// Design thumbnail
    pub thumbnail: Option<Thumbnail>,
    /// Design URLs
    pub urls: DesignUrls,
    /// Design creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Design last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Design type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignType {
    /// Design type ID
    pub id: String,
    /// Design type label
    pub label: String,
}

/// Design URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignUrls {
    /// Edit URL
    pub edit_url: String,
    /// View URL
    pub view_url: String,
}

/// Brand template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTemplate {
    /// Brand template ID
    pub id: String,
    /// Brand template name
    pub name: String,
    /// Brand template description
    pub description: Option<String>,
    /// Brand template tags
    pub tags: Vec<String>,
    /// Brand template thumbnail
    pub thumbnail: Option<Thumbnail>,
    /// Brand template URLs
    pub urls: BrandTemplateUrls,
    /// Whether the template has a dataset
    pub has_dataset: bool,
    /// Brand template creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Brand template last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Brand template URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTemplateUrls {
    /// Edit URL
    pub edit_url: String,
    /// View URL
    pub view_url: String,
}

/// Brand template dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTemplateDataset {
    /// Dataset fields
    pub fields: HashMap<String, DatasetField>,
}

/// Dataset field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetField {
    /// Field type
    #[serde(rename = "type")]
    pub field_type: DatasetFieldType,
    /// Field label
    pub label: String,
    /// Field description
    pub description: Option<String>,
    /// Whether the field is required
    pub required: bool,
}

/// Dataset field type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatasetFieldType {
    /// Text field
    Text,
    /// Image field
    Image,
    /// Chart field
    Chart,
}

/// Folder metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Folder ID
    pub id: String,
    /// Folder name
    pub name: String,
    /// Folder creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Folder last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,
    /// User email
    pub email: String,
    /// User display name
    pub display_name: String,
    /// User profile photo URL
    pub profile_photo_url: Option<String>,
    /// User team information
    pub team: Option<Team>,
}

/// Team information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// Team ID
    pub id: String,
    /// Team name
    pub name: String,
}

/// Comment thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentThread {
    /// Thread ID
    pub id: String,
    /// Thread type
    #[serde(rename = "type")]
    pub thread_type: CommentThreadType,
    /// Thread context
    pub context: CommentContext,
    /// Thread timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Thread author
    pub author: CommentAuthor,
    /// Thread content
    pub content: CommentContent,
    /// Thread replies
    pub replies: Vec<CommentReply>,
}

/// Comment thread type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentThreadType {
    /// Regular comment
    Comment,
    /// Suggestion
    Suggestion,
}

/// Comment context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentContext {
    /// Design ID
    pub design_id: String,
    /// Page number (0-indexed)
    pub page: u32,
    /// Position coordinates
    pub position: Option<CommentPosition>,
}

/// Comment position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentPosition {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// Comment author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    /// Author ID
    pub id: String,
    /// Author display name
    pub display_name: String,
    /// Author profile photo URL
    pub profile_photo_url: Option<String>,
}

/// Comment content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentContent {
    /// Comment text
    pub text: String,
}

/// Comment reply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    /// Reply ID
    pub id: String,
    /// Reply author
    pub author: CommentAuthor,
    /// Reply content
    pub content: CommentContent,
    /// Reply timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Export format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExportFormat {
    /// PNG format
    Png,
    /// JPG format
    Jpg,
    /// PDF format
    Pdf,
    /// SVG format
    Svg,
    /// GIF format
    Gif,
    /// MP4 format
    Mp4,
    /// PPTX format
    Pptx,
}

/// Export quality
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportQuality {
    /// Low quality
    Low,
    /// Medium quality
    Medium,
    /// High quality
    High,
}

/// Job status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Job is in progress
    #[serde(rename = "in_progress")]
    InProgress,
    /// Job completed successfully
    #[serde(rename = "success")]
    Success,
    /// Job failed
    #[serde(rename = "failed")]
    Failed,
}

/// Base job response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job<T> {
    /// Job ID
    pub id: String,
    /// Job status
    pub status: JobStatus,
    /// Job result (present when status is Success)
    pub result: Option<T>,
    /// Job error (present when status is Failed)
    pub error: Option<JobError>,
}

/// Asset upload job response (has different structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadJob {
    /// Job ID
    pub id: String,
    /// Job status
    pub status: JobStatus,
    /// Asset data (present when status is Success)
    pub asset: Option<Asset>,
    /// Job error (present when status is Failed)
    pub error: Option<JobError>,
}

/// Wrapper for job responses from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResponse<T> {
    /// The job data
    pub job: Job<T>,
}

/// Wrapper for asset upload job responses from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadJobResponse {
    /// The asset upload job data
    pub job: AssetUploadJob,
}

/// Job error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}

/// Asset upload job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadResult {
    /// Created asset
    pub asset: Asset,
}

/// Export job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// Export URLs
    pub urls: Vec<ExportUrl>,
}

/// Export URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportUrl {
    /// Page number
    pub page: u32,
    /// Export URL
    pub url: String,
}

/// Autofill job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutofillResult {
    /// Created design
    pub design: Design,
}

/// Ownership filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwnershipType {
    /// Owned by user
    Owned,
    /// Shared with user
    Shared,
    /// All accessible items
    All,
}

/// Sort order
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortByType {
    /// Sort by creation date (newest first)
    CreatedDescending,
    /// Sort by creation date (oldest first)
    CreatedAscending,
    /// Sort by last modified date (newest first)
    ModifiedDescending,
    /// Sort by last modified date (oldest first)
    ModifiedAscending,
    /// Sort by name (A-Z)
    NameAscending,
    /// Sort by name (Z-A)
    NameDescending,
}

/// Dataset filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatasetFilter {
    /// Only items with datasets
    WithDataset,
    /// Only items without datasets
    WithoutDataset,
    /// All items
    All,
}
