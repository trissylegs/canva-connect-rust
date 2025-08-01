//! Data models for the Canva Connect API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Design metadata (full details)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Design {
    /// Design ID
    pub id: String,
    /// Design title
    pub title: Option<String>,
    /// Design owner
    pub owner: TeamUserSummary,
    /// Design thumbnail
    pub thumbnail: Option<Thumbnail>,
    /// Design URLs
    pub urls: DesignLinks,
    /// Design creation timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Design last updated timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Total number of pages in the design
    pub page_count: Option<u32>,
}

/// Design summary (basic details without owner)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignSummary {
    /// Design ID
    pub id: String,
    /// Design title
    pub title: Option<String>,
    /// Design thumbnail
    pub thumbnail: Option<Thumbnail>,
    /// Design URLs
    pub urls: DesignLinks,
    /// Design creation timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Design last updated timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Total number of pages in the design
    pub page_count: Option<u32>,
}

/// Team user summary containing user and team IDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUserSummary {
    /// User ID
    pub user_id: String,
    /// Team ID
    pub team_id: String,
}

/// Design URLs for editing and viewing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignLinks {
    /// Temporary edit URL (valid for 30 days)
    pub edit_url: String,
    /// Temporary view URL (valid for 30 days)
    pub view_url: String,
}

/// Request to list designs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDesignsRequest {
    /// Search query
    pub query: Option<String>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
    /// Filter by ownership
    pub ownership: Option<OwnershipType>,
    /// Sort order
    pub sort_by: Option<SortByType>,
}

/// Response for listing designs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetListDesignResponse {
    /// List of designs
    pub items: Vec<Design>,
    /// Continuation token for next page
    pub continuation: Option<String>,
}

/// Request to create a design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDesignRequest {
    /// Design type configuration
    pub design_type: Option<DesignTypeInput>,
    /// Asset ID to insert into the design
    pub asset_id: Option<String>,
    /// Design title
    pub title: Option<String>,
}

/// Response for creating a design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDesignResponse {
    /// Created design
    pub design: Design,
}

/// Response for getting a design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDesignResponse {
    /// Design data
    pub design: Design,
}

/// Design type input for creating designs (tagged union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DesignTypeInput {
    /// Preset design type
    Preset {
        /// Preset design type name
        name: PresetDesignTypeName,
    },
    /// Custom design type
    Custom {
        /// Design width in pixels
        width: u32,
        /// Design height in pixels
        height: u32,
    },
}

/// Preset design type names
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresetDesignTypeName {
    /// Document
    Doc,
    /// Whiteboard
    Whiteboard,
    /// Presentation
    Presentation,
}

/// Ownership filter for designs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OwnershipType {
    /// Any designs (owned or shared)
    Any,
    /// Only owned designs
    Owned,
    /// Only shared designs
    Shared,
}

/// Sort order for designs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortByType {
    /// Sort by relevance
    Relevance,
    /// Sort by modified date (descending)
    ModifiedDescending,
    /// Sort by modified date (ascending)
    ModifiedAscending,
    /// Sort by title (descending)
    TitleDescending,
    /// Sort by title (ascending)
    TitleAscending,
}

/// Brand template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTemplate {
    /// Brand template ID
    pub id: String,
    /// Brand template title
    pub title: String,
    /// Brand template thumbnail
    pub thumbnail: Option<Thumbnail>,
    /// Brand template view URL
    pub view_url: String,
    /// Brand template create URL
    pub create_url: String,
    /// Brand template creation timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Brand template last updated timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
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
    /// Dataset fields (keyed by field name)
    pub dataset: HashMap<String, DataField>,
}

/// Dataset field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DataField {
    /// Text field
    Text {
        /// Field label
        label: Option<String>,
        /// Field description
        description: Option<String>,
        /// Whether the field is required
        required: Option<bool>,
    },
    /// Image field
    Image {
        /// Field label
        label: Option<String>,
        /// Field description
        description: Option<String>,
        /// Whether the field is required
        required: Option<bool>,
    },
    /// Chart field
    Chart {
        /// Field label
        label: Option<String>,
        /// Field description
        description: Option<String>,
        /// Whether the field is required
        required: Option<bool>,
    },
}

/// Folder metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Folder ID
    pub id: String,
    /// Folder name
    pub name: String,
    /// When the folder was created (Unix timestamp)
    pub created_at: i64,
    /// When the folder was last updated (Unix timestamp)
    pub updated_at: i64,
    /// Folder thumbnail (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
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
    /// Design ID
    pub design_id: String,
    /// Thread type
    pub thread_type: CommentThreadType,
    /// Thread author
    pub author: Option<SimpleUser>,
    /// Thread creation timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Thread last updated timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Simple user information for comments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleUser {
    /// User ID
    pub id: String,
    /// User display name
    pub display_name: String,
}

/// Comment thread type (tagged union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CommentThreadType {
    /// Regular comment
    Comment {
        /// Comment content
        content: CommentContent,
        /// User mentions in the comment
        mentions: std::collections::HashMap<String, UserMention>,
        /// Assigned user
        assignee: Option<SimpleUser>,
        /// User who resolved the comment
        resolver: Option<SimpleUser>,
    },
    /// Suggestion
    Suggestion {
        /// Suggested edits
        suggested_edits: Vec<SuggestedEdit>,
        /// Suggestion status
        status: SuggestionStatus,
    },
}

/// Comment content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentContent {
    /// Comment content in plaintext
    pub plaintext: String,
    /// Comment content in markdown (optional)
    pub markdown: Option<String>,
}

/// Comment reply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    /// Reply ID
    pub id: String,
    /// Reply author
    pub author: Option<SimpleUser>,
    /// Reply content
    pub content: CommentContent,
    /// Reply timestamp (Unix timestamp in seconds)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// User mentions in the reply
    pub mentions: std::collections::HashMap<String, UserMention>,
}

/// User mention in a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMention {
    /// The mention tag in the format user_id:team_id
    pub tag: String,
    /// The mentioned user
    pub user: TeamUserSummary,
}

/// Suggested edit in a suggestion thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedEdit {
    /// Edit ID
    pub id: String,
    /// Edit type
    #[serde(rename = "type")]
    pub edit_type: String,
    /// Edit description
    pub description: String,
}

/// Suggestion status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionStatus {
    /// Suggestion is pending
    Pending,
    /// Suggestion has been accepted
    Accepted,
    /// Suggestion has been rejected
    Rejected,
}

/// Response from creating a comment thread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadResponse {
    /// The created thread
    pub thread: CommentThread,
}

/// Export format (tagged union)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ExportFormat {
    /// PDF format
    Pdf {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// Paper size
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<ExportPageSize>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
    /// JPG format
    Jpg {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// JPEG compression quality (1-100)
        quality: u8,
        /// Height in pixels
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Width in pixels
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
    /// PNG format
    Png {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// Height in pixels
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<u32>,
        /// Width in pixels
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
    /// PPTX format
    Pptx {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
    /// GIF format
    Gif {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
    /// MP4 format
    Mp4 {
        /// Export quality
        #[serde(skip_serializing_if = "Option::is_none")]
        export_quality: Option<ExportQuality>,
        /// Pages to export (1-indexed)
        #[serde(skip_serializing_if = "Option::is_none")]
        pages: Option<Vec<u32>>,
    },
}

/// Export page size for PDF exports
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportPageSize {
    /// A4 paper size
    A4,
    /// A3 paper size
    A3,
    /// Letter paper size
    Letter,
    /// Legal paper size
    Legal,
}

/// Export quality
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportQuality {
    /// Regular quality
    Regular,
    /// Pro quality
    Pro,
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

/// Export job containing status and results
pub type ExportJob = Job<ExportResult>;

/// Folder item summary (tagged union for different item types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FolderItemSummary {
    /// Folder item
    Folder {
        /// Folder details
        folder: Folder,
    },
    /// Design item
    Design {
        /// Design details
        design: DesignSummary,
    },
    /// Image item
    Image {
        /// Image details
        image: Asset, // Using Asset for now, could be specific ImageItem
    },
}

/// Legacy folder item for compatibility
pub type FolderItem = FolderItemSummary;

/// Autofill job result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutofillResult {
    /// Created design
    pub design: Design,
}

/// Request to create a design autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDesignAutofillJobRequest {
    /// ID of the input brand template
    pub brand_template_id: String,
    /// Title to use for the autofilled design
    pub title: Option<String>,
    /// Data object containing the data fields and values to autofill
    pub data: HashMap<String, DatasetValue>,
}

/// Response from creating a design autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDesignAutofillJobResponse {
    /// The autofill job
    pub job: DesignAutofillJob,
}

/// Response from getting a design autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDesignAutofillJobResponse {
    /// The autofill job
    pub job: DesignAutofillJob,
}

/// Details about the autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignAutofillJob {
    /// ID of the asynchronous job
    pub id: String,
    /// Status of the design autofill job
    pub status: DesignAutofillStatus,
    /// Result of the design autofill job (present when status is success)
    pub result: Option<DesignAutofillJobResult>,
    /// Error details (present when status is failed)
    pub error: Option<AutofillError>,
}

/// Status of the design autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesignAutofillStatus {
    /// Job is still in progress
    InProgress,
    /// Job completed successfully
    Success,
    /// Job failed
    Failed,
}

/// Result of the design autofill job
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DesignAutofillJobResult {
    /// Design has been created and saved to user's root folder
    CreateDesign {
        /// The created design
        design: Design,
    },
}

/// If the autofill job fails, this object provides details about the error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutofillError {
    /// Error code
    pub code: AutofillErrorCode,
    /// A human-readable description of what went wrong
    pub message: String,
}

/// Autofill error codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutofillErrorCode {
    /// General autofill error
    AutofillError,
    /// Thumbnail generation error
    ThumbnailGenerationError,
    /// Create design error
    CreateDesignError,
}

/// The data field to autofill
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DatasetValue {
    /// Image data field
    Image {
        /// Asset ID of the image to insert
        asset_id: String,
    },
    /// Text data field
    Text {
        /// Text to insert into the template element
        text: String,
    },
    /// Chart data field (preview feature)
    Chart {
        /// Chart data
        chart_data: DataTable,
    },
}

/// Tabular data, structured in rows of cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    /// Rows of data (first row usually contains column headers)
    pub rows: Vec<DataTableRow>,
}

/// A single row of tabular data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTableRow {
    /// Cells of data in row (all rows must have the same number of cells)
    pub cells: Vec<DataTableCell>,
}

/// A single tabular data cell
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataTableCell {
    /// String data cell
    String {
        /// String value
        value: Option<String>,
    },
    /// Number data cell
    Number {
        /// Number value
        value: Option<f64>,
    },
    /// Boolean data cell
    Boolean {
        /// Boolean value
        value: Option<bool>,
    },
    /// Date data cell (Unix timestamp in seconds)
    Date {
        /// Date value as Unix timestamp
        value: Option<i64>,
    },
}

/// Dataset filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetFilter {
    /// Any items
    Any,
    /// Only items with non-empty datasets
    NonEmpty,
    /// Only items with empty datasets
    Empty,
}
