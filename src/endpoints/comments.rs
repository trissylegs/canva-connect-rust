//! Comments API endpoints
//!
//! This module provides access to the Canva Comments API, allowing you to
//! create and manage comments and comment threads on designs.
//!
//! **Note:** The Comments API is currently in preview and includes both
//! deprecated and new endpoints. This implementation focuses on the newer
//! thread-based API.

use crate::{
    client::Client,
    error::Result,
    models::{CommentReply, CommentThread, CreateThreadResponse},
};
use serde::{Deserialize, Serialize};

/// Client for the Comments API
#[derive(Debug, Clone)]
pub struct CommentsApi {
    client: Client,
}

/// Request to create a new comment thread
#[derive(Debug, Clone, Serialize)]
pub struct CreateThreadRequest {
    /// The comment message in plaintext
    pub message_plaintext: String,
    /// Optional assignee ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
}

/// Object to attach a comment to
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CommentObjectInput {
    /// Design comment object
    Design {
        /// The ID of the design
        design_id: String,
    },
}

/// Request to create a reply to a comment thread
#[derive(Debug, Clone, Serialize)]
pub struct CreateReplyRequest {
    /// The reply comment message in plaintext
    pub message_plaintext: String,
}

/// Response from creating a reply
#[derive(Debug, Clone, Deserialize)]
pub struct CreateReplyResponse {
    /// The created reply
    pub reply: CommentReply,
}

/// Response from getting a thread
#[derive(Debug, Clone, Deserialize)]
pub struct GetThreadResponse {
    /// The thread
    pub thread: CommentThread,
}

/// Response from getting a reply
#[derive(Debug, Clone, Deserialize)]
pub struct GetReplyResponse {
    /// The reply
    pub reply: CommentReply,
}

/// Response from listing replies
#[derive(Debug, Clone, Deserialize)]
pub struct ListRepliesResponse {
    /// List of replies
    pub items: Vec<CommentReply>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

/// Request parameters for listing replies
#[derive(Debug, Clone, Default)]
pub struct ListRepliesRequest {
    /// Maximum number of results to return (1-100)
    pub limit: Option<u32>,
    /// Continuation token for pagination
    pub continuation: Option<String>,
}

impl CommentsApi {
    /// Create a new comments API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new comment thread on a design
    ///
    /// **Required OAuth scope:** `comment:write`
    ///
    /// **Note:** This API is currently in preview and may have breaking changes.
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn create_thread(
        &self,
        design_id: &str,
        request: &CreateThreadRequest,
    ) -> Result<CreateThreadResponse> {
        let url = format!("/v1/designs/{design_id}/comments");
        let response = self.client.post(&url, request).await?;
        Ok(response.json::<CreateThreadResponse>().await?)
    }

    /// Get a comment thread
    ///
    /// **Required OAuth scope:** `comment:read`
    ///
    /// **Note:** This API is currently in preview and may have breaking changes.
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_thread(&self, design_id: &str, thread_id: &str) -> Result<GetThreadResponse> {
        let url = format!("/v1/designs/{design_id}/comments/{thread_id}");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetThreadResponse>().await?)
    }

    /// Create a reply to a comment thread
    ///
    /// **Required OAuth scope:** `comment:write`
    ///
    /// **Note:** This API is currently in preview and may have breaking changes.
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn create_reply(
        &self,
        design_id: &str,
        thread_id: &str,
        request: &CreateReplyRequest,
    ) -> Result<CreateReplyResponse> {
        let url = format!("/v1/designs/{design_id}/comments/{thread_id}/replies");
        let response = self.client.post(&url, request).await?;
        Ok(response.json::<CreateReplyResponse>().await?)
    }

    /// Get a specific reply
    ///
    /// **Required OAuth scope:** `comment:read`
    ///
    /// **Note:** This API is currently in preview and may have breaking changes.
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_reply(
        &self,
        design_id: &str,
        thread_id: &str,
        reply_id: &str,
    ) -> Result<GetReplyResponse> {
        let url = format!("/v1/designs/{design_id}/comments/{thread_id}/replies/{reply_id}");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetReplyResponse>().await?)
    }

    /// List replies for a comment thread
    ///
    /// **Required OAuth scope:** `comment:read`
    ///
    /// **Note:** This API is currently in preview and may have breaking changes.
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn list_replies(
        &self,
        design_id: &str,
        thread_id: &str,
        request: &ListRepliesRequest,
    ) -> Result<ListRepliesResponse> {
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
            format!("/v1/designs/{design_id}/comments/{thread_id}/replies")
        } else {
            format!(
                "/v1/designs/{}/comments/{}/replies?{}",
                design_id,
                thread_id,
                query_params.join("&")
            )
        };

        let response = self.client.get(&url).await?;
        Ok(response.json::<ListRepliesResponse>().await?)
    }
}
