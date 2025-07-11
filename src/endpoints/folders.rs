//! Folders API endpoints
//!
//! This module provides access to the Canva Folders API, allowing you to
//! organize and manage content in folders.

use crate::{
    client::Client,
    error::Result,
    models::{Folder, FolderItem},
};
use serde::{Deserialize, Serialize};

/// Client for the Folders API
#[derive(Debug, Clone)]
pub struct FoldersApi {
    client: Client,
}

/// Request to create a folder
#[derive(Debug, Clone, Serialize)]
pub struct CreateFolderRequest {
    /// The folder name
    pub name: String,
    /// Parent folder ID (use "root" for top-level folders)
    pub parent_folder_id: String,
}

/// Response from creating a folder
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFolderResponse {
    /// The created folder
    pub folder: Folder,
}

/// Request to update a folder
#[derive(Debug, Clone, Serialize)]
pub struct UpdateFolderRequest {
    /// The new folder name
    pub name: String,
}

/// Response from updating a folder
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFolderResponse {
    /// The updated folder
    pub folder: Folder,
}

/// Response from getting a folder
#[derive(Debug, Clone, Deserialize)]
pub struct GetFolderResponse {
    /// The folder
    pub folder: Folder,
}

/// Response from listing folder items
#[derive(Debug, Clone, Deserialize)]
pub struct ListFolderItemsResponse {
    /// List of folder items
    pub items: Vec<FolderItem>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

/// Request to move a folder item
#[derive(Debug, Clone, Serialize)]
pub struct MoveFolderItemRequest {
    /// The item ID to move
    pub item_id: String,
    /// The destination folder ID
    pub destination_folder_id: String,
}

/// Parameters for listing folder items
#[derive(Debug, Clone, Default)]
pub struct ListFolderItemsRequest {
    /// Maximum number of results to return
    pub limit: Option<u32>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}

impl FoldersApi {
    /// Create a new folders API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new folder
    ///
    /// **Required OAuth scope:** `folder:write`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn create_folder(
        &self,
        request: &CreateFolderRequest,
    ) -> Result<CreateFolderResponse> {
        let response = self.client.post("/v1/folders", request).await?;
        Ok(response.json::<CreateFolderResponse>().await?)
    }

    /// Get a folder by ID
    ///
    /// **Required OAuth scope:** `folder:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_folder(&self, folder_id: &str) -> Result<GetFolderResponse> {
        let url = format!("/v1/folders/{folder_id}");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetFolderResponse>().await?)
    }

    /// Update a folder
    ///
    /// **Required OAuth scope:** `folder:write`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn update_folder(
        &self,
        folder_id: &str,
        request: &UpdateFolderRequest,
    ) -> Result<UpdateFolderResponse> {
        let url = format!("/v1/folders/{folder_id}");
        let response = self.client.put(&url, request).await?;
        Ok(response.json::<UpdateFolderResponse>().await?)
    }

    /// List items in a folder
    ///
    /// **Required OAuth scope:** `folder:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn list_folder_items(
        &self,
        folder_id: &str,
        request: &ListFolderItemsRequest,
    ) -> Result<ListFolderItemsResponse> {
        let mut query_params = Vec::new();

        if let Some(limit) = request.limit {
            query_params.push(format!("limit={limit}"));
        }

        if let Some(continuation) = &request.continuation {
            query_params.push(format!(
                "continuation={}",
                urlencoding::encode(continuation)
            ));
        }

        let url = if query_params.is_empty() {
            format!("/v1/folders/{folder_id}/items")
        } else {
            format!("/v1/folders/{}/items?{}", folder_id, query_params.join("&"))
        };

        let response = self.client.get(&url).await?;
        Ok(response.json::<ListFolderItemsResponse>().await?)
    }

    /// Move a folder item
    ///
    /// **Required OAuth scope:** `folder:write`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn move_folder_item(&self, request: &MoveFolderItemRequest) -> Result<()> {
        let _response = self.client.post("/v1/folders/move", request).await?;
        // The client already handles error responses, so if we get here, it's successful
        Ok(())
    }
}
