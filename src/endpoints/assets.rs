//! Assets API endpoints for the Canva Connect API.
//!
//! This module provides access to asset management operations including:
//! - Getting asset metadata
//! - Updating asset properties (name and tags)
//! - Deleting assets
//! - Creating asset upload jobs
//! - Managing URL-based asset uploads
//!
//! ## Available Operations
//!
//! - [`get`](AssetsApi::get) - Get metadata for a specific asset
//! - [`update`](AssetsApi::update) - Update asset name and tags
//! - [`delete`](AssetsApi::delete) - Delete an asset (moves to trash)
//! - [`create_upload_job`](AssetsApi::create_upload_job) - Upload asset from binary data
//! - [`create_url_upload_job`](AssetsApi::create_url_upload_job) - Upload asset from URL
//! - [`get_upload_job`](AssetsApi::get_upload_job) - Check upload job status
//! - [`wait_for_upload_job`](AssetsApi::wait_for_upload_job) - Wait for upload completion
//!
//! ## Note on Asset Listing
//!
//! The Canva Connect API does not provide a general asset listing endpoint.
//! Assets are typically accessed through other endpoints like designs or
//! by their specific asset IDs.

use crate::{client::Client, error::Result, models::*};
use serde::{Deserialize, Serialize};

/// Assets API client
#[derive(Debug, Clone)]
pub struct AssetsApi {
    client: Client,
}

impl AssetsApi {
    /// Create a new assets API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Get a specific asset by ID
    pub async fn get(&self, asset_id: &str) -> Result<Asset> {
        let path = format!("/v1/assets/{asset_id}");
        let response: GetAssetResponse = self.client.get_json(&path).await?;
        Ok(response.asset)
    }

    /// Update an asset (name and tags)
    pub async fn update(&self, asset_id: &str, request: UpdateAssetRequest) -> Result<Asset> {
        let path = format!("/v1/assets/{asset_id}");
        let response: UpdateAssetResponse = self.client.patch_json(&path, &request).await?;
        Ok(response.asset)
    }

    /// Delete an asset
    pub async fn delete(&self, asset_id: &str) -> Result<()> {
        let path = format!("/v1/assets/{asset_id}");
        self.client.delete(&path).await?;
        Ok(())
    }

    /// Create an asset upload job
    pub async fn create_upload_job(
        &self,
        file_data: Vec<u8>,
        metadata: AssetUploadMetadata,
    ) -> Result<crate::models::AssetUploadJob> {
        let metadata_json = serde_json::to_string(&metadata)?;
        let response = self
            .client
            .upload_file("/v1/asset-uploads", file_data, Some(&metadata_json))
            .await?;
        let job_response: crate::models::AssetUploadJobResponse = response.json().await?;
        Ok(job_response.job)
    }

    /// Get the status of an asset upload job
    pub async fn get_upload_job(&self, job_id: &str) -> Result<crate::models::AssetUploadJob> {
        let path = format!("/v1/asset-uploads/{job_id}");
        let response: crate::models::AssetUploadJobResponse = self.client.get_json(&path).await?;
        Ok(response.job)
    }

    /// Create an asset upload job from URL
    pub async fn create_url_upload_job(
        &self,
        request: CreateUrlAssetUploadJobRequest,
    ) -> Result<crate::models::AssetUploadJob> {
        let response: crate::models::AssetUploadJobResponse = self
            .client
            .post_json("/v1/url-asset-uploads", &request)
            .await?;
        Ok(response.job)
    }

    /// Get the status of a URL asset upload job
    pub async fn get_url_upload_job(&self, job_id: &str) -> Result<crate::models::AssetUploadJob> {
        let path = format!("/v1/url-asset-uploads/{job_id}");
        let response: crate::models::AssetUploadJobResponse = self.client.get_json(&path).await?;
        Ok(response.job)
    }

    /// Wait for an upload job to complete
    pub async fn wait_for_upload_job(&self, job_id: &str) -> Result<crate::models::Asset> {
        loop {
            let job = self.get_upload_job(job_id).await?;

            match job.status {
                JobStatus::Success => {
                    return job.asset.ok_or_else(|| {
                        crate::error::Error::Generic("Job succeeded but no asset data".to_string())
                    });
                }
                JobStatus::Failed => {
                    let error_msg = job
                        .error
                        .map(|e| format!("{}: {}", e.code, e.message))
                        .unwrap_or_else(|| "Job failed with unknown error".to_string());
                    return Err(crate::error::Error::Generic(error_msg));
                }
                JobStatus::InProgress => {
                    // Wait a bit before polling again
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    /// Wait for a URL upload job to complete
    pub async fn wait_for_url_upload_job(&self, job_id: &str) -> Result<crate::models::Asset> {
        loop {
            let job = self.get_url_upload_job(job_id).await?;

            match job.status {
                JobStatus::Success => {
                    return job.asset.ok_or_else(|| {
                        crate::error::Error::Generic("Job succeeded but no asset data".to_string())
                    });
                }
                JobStatus::Failed => {
                    let error_msg = job
                        .error
                        .map(|e| format!("{}: {}", e.code, e.message))
                        .unwrap_or_else(|| "Job failed with unknown error".to_string());
                    return Err(crate::error::Error::Generic(error_msg));
                }
                JobStatus::InProgress => {
                    // Wait a bit before polling again
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }
    }
}

/// Asset upload metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadMetadata {
    /// Asset name, encoded in Base64
    pub name_base64: String,
    /// Asset tags
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl AssetUploadMetadata {
    /// Create new metadata with name automatically Base64 encoded
    pub fn new(name: &str, tags: Vec<String>) -> Self {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        Self {
            name_base64: STANDARD.encode(name.as_bytes()),
            tags,
        }
    }
}

/// Request to create URL asset upload job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUrlAssetUploadJobRequest {
    /// URL to upload from
    pub url: String,
    /// Upload metadata
    pub upload_metadata: AssetUploadMetadata,
}

/// Request to update an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAssetRequest {
    /// New asset name
    pub name: Option<String>,
    /// New asset tags
    pub tags: Option<Vec<String>>,
}

/// Response from getting an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAssetResponse {
    /// The asset
    pub asset: Asset,
}

/// Response from updating an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAssetResponse {
    /// The updated asset
    pub asset: Asset,
}
