//! Folders API endpoints
//!
//! This module provides access to the Canva Folders API, allowing you to
//! organize and manage content in folders.

use crate::{
    client::Client,
    error::Result,
    models::{Folder, FolderItemSummary},
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
    pub items: Vec<FolderItemSummary>,
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
    pub to_folder_id: String,
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
        let response = self.client.patch(&url, request).await?;
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::auth::AccessToken;

    #[test]
    fn test_folders_api_creation() {
        let access_token = AccessToken::new("test_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");

        let _folders_api = client.folders();
    }

    #[test]
    fn test_create_folder_request_creation() {
        let request = CreateFolderRequest {
            name: "My Project".to_string(),
            parent_folder_id: "root".to_string(),
        };

        assert_eq!(request.name, "My Project");
        assert_eq!(request.parent_folder_id, "root");
    }

    #[test]
    fn test_create_folder_request_with_parent() {
        let request = CreateFolderRequest {
            name: "Subfolder".to_string(),
            parent_folder_id: "folder_123".to_string(),
        };

        assert_eq!(request.name, "Subfolder");
        assert_eq!(request.parent_folder_id, "folder_123");
    }

    #[test]
    fn test_create_folder_request_serialization() {
        let request = CreateFolderRequest {
            name: "Test Folder".to_string(),
            parent_folder_id: "root".to_string(),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"name\":\"Test Folder\""));
        assert!(serialized.contains("\"parent_folder_id\":\"root\""));
    }

    #[test]
    fn test_update_folder_request_creation() {
        let request = UpdateFolderRequest {
            name: "Updated Folder Name".to_string(),
        };

        assert_eq!(request.name, "Updated Folder Name");
    }

    #[test]
    fn test_update_folder_request_serialization() {
        let request = UpdateFolderRequest {
            name: "New Name".to_string(),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"name\":\"New Name\""));
    }

    #[test]
    fn test_move_folder_item_request_creation() {
        let request = MoveFolderItemRequest {
            item_id: "item_123".to_string(),
            to_folder_id: "folder_456".to_string(),
        };

        assert_eq!(request.item_id, "item_123");
        assert_eq!(request.to_folder_id, "folder_456");
    }

    #[test]
    fn test_move_folder_item_request_serialization() {
        let request = MoveFolderItemRequest {
            item_id: "design_789".to_string(),
            to_folder_id: "root".to_string(),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"item_id\":\"design_789\""));
        assert!(serialized.contains("\"to_folder_id\":\"root\""));
    }

    #[test]
    fn test_list_folder_items_request_default() {
        let request = ListFolderItemsRequest::default();

        assert!(request.limit.is_none());
        assert!(request.continuation.is_none());
    }

    #[test]
    fn test_list_folder_items_request_with_limit() {
        let request = ListFolderItemsRequest {
            limit: Some(50),
            continuation: None,
        };

        assert_eq!(request.limit, Some(50));
        assert!(request.continuation.is_none());
    }

    #[test]
    fn test_list_folder_items_request_with_continuation() {
        let request = ListFolderItemsRequest {
            limit: Some(25),
            continuation: Some("next_token_123".to_string()),
        };

        assert_eq!(request.limit, Some(25));
        assert_eq!(request.continuation, Some("next_token_123".to_string()));
    }

    #[test]
    fn test_create_folder_request_with_special_characters() {
        let request = CreateFolderRequest {
            name: "Folder with émojis 🎨📁".to_string(),
            parent_folder_id: "parent_folder_456".to_string(),
        };

        assert_eq!(request.name, "Folder with émojis 🎨📁");
        assert_eq!(request.parent_folder_id, "parent_folder_456");
    }

    #[test]
    fn test_create_folder_request_with_empty_name() {
        let request = CreateFolderRequest {
            name: "".to_string(),
            parent_folder_id: "root".to_string(),
        };

        assert!(request.name.is_empty());
        assert_eq!(request.parent_folder_id, "root");

        // Should still serialize properly
        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"name\":\"\""));
    }

    #[test]
    fn test_update_folder_request_with_long_name() {
        let long_name = "Very long folder name ".repeat(10);
        let request = UpdateFolderRequest {
            name: long_name.clone(),
        };

        assert_eq!(request.name, long_name);
        assert!(request.name.len() > 200);
    }

    #[test]
    fn test_folders_api_debug() {
        let access_token = AccessToken::new("debug_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let folders_api = client.folders();

        let debug_str = format!("{folders_api:?}");
        assert!(debug_str.contains("FoldersApi"));
    }

    #[test]
    fn test_folders_api_clone() {
        let access_token = AccessToken::new("clone_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let folders_api = client.folders();

        let cloned_api = folders_api.clone();

        // Both should be separate instances but functionally equivalent
        let original_debug = format!("{folders_api:?}");
        let cloned_debug = format!("{cloned_api:?}");
        assert_eq!(original_debug, cloned_debug);
    }

    #[test]
    fn test_create_folder_request_debug_format() {
        let request = CreateFolderRequest {
            name: "Debug Test Folder".to_string(),
            parent_folder_id: "debug_parent".to_string(),
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("CreateFolderRequest"));
        assert!(debug_str.contains("Debug Test Folder"));
        assert!(debug_str.contains("debug_parent"));
    }

    #[test]
    fn test_update_folder_request_debug_format() {
        let request = UpdateFolderRequest {
            name: "Debug Update Name".to_string(),
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("UpdateFolderRequest"));
        assert!(debug_str.contains("Debug Update Name"));
    }

    #[test]
    fn test_move_folder_item_request_debug_format() {
        let request = MoveFolderItemRequest {
            item_id: "debug_item".to_string(),
            to_folder_id: "debug_destination".to_string(),
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("MoveFolderItemRequest"));
        assert!(debug_str.contains("debug_item"));
        assert!(debug_str.contains("debug_destination"));
    }

    #[test]
    fn test_list_folder_items_request_debug_format() {
        let request = ListFolderItemsRequest {
            limit: Some(42),
            continuation: Some("debug_continuation".to_string()),
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("ListFolderItemsRequest"));
        assert!(debug_str.contains("42"));
        assert!(debug_str.contains("debug_continuation"));
    }

    #[test]
    fn test_move_folder_item_to_root() {
        let request = MoveFolderItemRequest {
            item_id: "some_design_id".to_string(),
            to_folder_id: "root".to_string(),
        };

        assert_eq!(request.item_id, "some_design_id");
        assert_eq!(request.to_folder_id, "root");

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"to_folder_id\":\"root\""));
    }

    #[test]
    fn test_list_folder_items_request_edge_cases() {
        // Test with maximum practical limit
        let high_limit_request = ListFolderItemsRequest {
            limit: Some(1000),
            continuation: None,
        };
        assert_eq!(high_limit_request.limit, Some(1000));

        // Test with minimum limit
        let low_limit_request = ListFolderItemsRequest {
            limit: Some(1),
            continuation: None,
        };
        assert_eq!(low_limit_request.limit, Some(1));
    }

    #[test]
    fn test_create_folder_request_serialization_structure() {
        let request = CreateFolderRequest {
            name: "Structure Test".to_string(),
            parent_folder_id: "test_parent".to_string(),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");

        // Verify JSON structure
        let json: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        assert_eq!(json["name"], "Structure Test");
        assert_eq!(json["parent_folder_id"], "test_parent");
    }

    #[test]
    fn test_all_request_types_with_unicode() {
        // Test Unicode handling across all request types
        let unicode_name = "测试文件夹 🌍";
        let unicode_id = "フォルダー_123";

        let create_request = CreateFolderRequest {
            name: unicode_name.to_string(),
            parent_folder_id: unicode_id.to_string(),
        };

        let update_request = UpdateFolderRequest {
            name: unicode_name.to_string(),
        };

        let move_request = MoveFolderItemRequest {
            item_id: unicode_id.to_string(),
            to_folder_id: "root".to_string(),
        };

        // All should handle Unicode properly
        assert_eq!(create_request.name, unicode_name);
        assert_eq!(update_request.name, unicode_name);
        assert_eq!(move_request.item_id, unicode_id);

        // All should serialize properly
        let create_serialized =
            serde_json::to_string(&create_request).expect("Failed to serialize create request");
        let update_serialized =
            serde_json::to_string(&update_request).expect("Failed to serialize update request");
        let move_serialized =
            serde_json::to_string(&move_request).expect("Failed to serialize move request");

        assert!(create_serialized.contains(unicode_name));
        assert!(update_serialized.contains(unicode_name));
        assert!(move_serialized.contains(unicode_id));
    }
}
