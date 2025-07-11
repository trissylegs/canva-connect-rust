//! Brand Templates API endpoints
//!
//! This module provides access to the Canva Brand Templates API, allowing you to
//! list brand templates and retrieve their datasets.

use crate::{
    client::Client,
    error::Result,
    models::{BrandTemplate, BrandTemplateDataset},
};
use serde::{Deserialize, Serialize};

/// Client for the Brand Templates API
#[derive(Debug, Clone)]
pub struct BrandTemplatesApi {
    client: Client,
}

/// Request body for brand template queries
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListBrandTemplatesRequest {
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    /// Maximum number of results to return (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Response from listing brand templates
#[derive(Debug, Clone, Deserialize)]
pub struct ListBrandTemplatesResponse {
    /// List of brand templates
    pub items: Vec<BrandTemplate>,
    /// Continuation token for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

/// Response from getting a brand template
#[derive(Debug, Clone, Deserialize)]
pub struct GetBrandTemplateResponse {
    /// The brand template
    pub brand_template: BrandTemplate,
}

/// Response from getting a brand template dataset
#[derive(Debug, Clone, Deserialize)]
pub struct GetBrandTemplateDatasetResponse {
    /// The brand template dataset
    pub dataset: BrandTemplateDataset,
}

impl BrandTemplatesApi {
    /// Create a new brand templates API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List brand templates
    ///
    /// Returns a list of brand templates in the user's team.
    ///
    /// **Required OAuth scope:** `brandtemplate:meta:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn list(
        &self,
        request: &ListBrandTemplatesRequest,
    ) -> Result<ListBrandTemplatesResponse> {
        let mut query_params = Vec::new();

        if let Some(continuation) = &request.continuation {
            query_params.push(format!(
                "continuation={}",
                urlencoding::encode(continuation)
            ));
        }

        if let Some(limit) = request.limit {
            query_params.push(format!("limit={limit}"));
        }

        let url = if query_params.is_empty() {
            "/v1/brand-templates".to_string()
        } else {
            format!("/v1/brand-templates?{}", query_params.join("&"))
        };

        let response = self.client.get(&url).await?;

        Ok(response.json::<ListBrandTemplatesResponse>().await?)
    }

    /// Get a specific brand template by ID
    ///
    /// Returns the details of a specific brand template.
    ///
    /// **Required OAuth scope:** `brandtemplate:meta:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get(&self, brand_template_id: &str) -> Result<GetBrandTemplateResponse> {
        let url = format!("/v1/brand-templates/{brand_template_id}");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetBrandTemplateResponse>().await?)
    }

    /// Get a brand template's dataset
    ///
    /// Returns the dataset for a specific brand template, which contains the
    /// data fields that can be used for autofill operations.
    ///
    /// **Required OAuth scope:** `brandtemplate:content:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_dataset(
        &self,
        brand_template_id: &str,
    ) -> Result<GetBrandTemplateDatasetResponse> {
        let url = format!("/v1/brand-templates/{brand_template_id}/dataset");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetBrandTemplateDatasetResponse>().await?)
    }
}
