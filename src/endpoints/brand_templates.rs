//! Brand Templates API endpoints
//!
//! This module provides access to the Canva Brand Templates API, allowing you to
//! list brand templates and retrieve their datasets.

use crate::{
    client::Client,
    error::Result,
    models::{BrandTemplate, DataField},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Client for the Brand Templates API
#[derive(Debug, Clone)]
pub struct BrandTemplatesApi {
    client: Client,
}

/// Request body for brand template queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    /// The brand template dataset fields (keyed by field name)
    pub dataset: HashMap<String, DataField>,
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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::auth::AccessToken;

    #[test]
    fn test_brand_templates_api_creation() {
        let access_token = AccessToken::new("test_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");

        let _brand_templates_api = client.brand_templates();
    }

    #[test]
    fn test_list_brand_templates_request_default() {
        let request = ListBrandTemplatesRequest::default();

        assert!(request.continuation.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_list_brand_templates_request_with_limit() {
        let request = ListBrandTemplatesRequest {
            continuation: None,
            limit: Some(50),
        };

        assert!(request.continuation.is_none());
        assert_eq!(request.limit, Some(50));
    }

    #[test]
    fn test_list_brand_templates_request_with_continuation() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("next_page_token".to_string()),
            limit: Some(25),
        };

        assert_eq!(request.continuation, Some("next_page_token".to_string()));
        assert_eq!(request.limit, Some(25));
    }

    #[test]
    fn test_list_brand_templates_request_serialization_empty() {
        let request = ListBrandTemplatesRequest::default();

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        // Default values should result in empty JSON object since all fields have skip_serializing_if
        assert_eq!(serialized, "{}");
    }

    #[test]
    fn test_list_brand_templates_request_serialization_with_values() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("test_token".to_string()),
            limit: Some(100),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"continuation\":\"test_token\""));
        assert!(serialized.contains("\"limit\":100"));
    }

    #[test]
    fn test_list_brand_templates_request_serialization_limit_only() {
        let request = ListBrandTemplatesRequest {
            continuation: None,
            limit: Some(10),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(!serialized.contains("continuation"));
        assert!(serialized.contains("\"limit\":10"));
    }

    #[test]
    fn test_list_brand_templates_request_serialization_continuation_only() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("abc123".to_string()),
            limit: None,
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"continuation\":\"abc123\""));
        assert!(!serialized.contains("limit"));
    }

    #[test]
    fn test_list_brand_templates_request_edge_cases() {
        // Test with minimum limit
        let min_request = ListBrandTemplatesRequest {
            continuation: None,
            limit: Some(1),
        };
        assert_eq!(min_request.limit, Some(1));

        // Test with maximum limit
        let max_request = ListBrandTemplatesRequest {
            continuation: None,
            limit: Some(100),
        };
        assert_eq!(max_request.limit, Some(100));

        // Test with empty continuation token
        let empty_continuation_request = ListBrandTemplatesRequest {
            continuation: Some("".to_string()),
            limit: None,
        };
        assert_eq!(
            empty_continuation_request.continuation,
            Some("".to_string())
        );
    }

    #[test]
    fn test_list_brand_templates_request_with_special_characters() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("token_with_special_chars_@#$%_🎨".to_string()),
            limit: Some(42),
        };

        assert_eq!(
            request.continuation,
            Some("token_with_special_chars_@#$%_🎨".to_string())
        );
        assert_eq!(request.limit, Some(42));

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("token_with_special_chars_@#$%_🎨"));
    }

    #[test]
    fn test_brand_templates_api_debug() {
        let access_token = AccessToken::new("debug_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let brand_templates_api = client.brand_templates();

        let debug_str = format!("{brand_templates_api:?}");
        assert!(debug_str.contains("BrandTemplatesApi"));
    }

    #[test]
    fn test_brand_templates_api_clone() {
        let access_token = AccessToken::new("clone_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let brand_templates_api = client.brand_templates();

        let cloned_api = brand_templates_api.clone();

        // Both should be separate instances but functionally equivalent
        let original_debug = format!("{brand_templates_api:?}");
        let cloned_debug = format!("{cloned_api:?}");
        assert_eq!(original_debug, cloned_debug);
    }

    #[test]
    fn test_list_brand_templates_request_debug_format() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("debug_continuation".to_string()),
            limit: Some(75),
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("ListBrandTemplatesRequest"));
        assert!(debug_str.contains("debug_continuation"));
        assert!(debug_str.contains("75"));
    }

    #[test]
    fn test_list_brand_templates_request_clone() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("original_token".to_string()),
            limit: Some(30),
        };

        let cloned_request = request.clone();

        assert_eq!(request.continuation, cloned_request.continuation);
        assert_eq!(request.limit, cloned_request.limit);

        // Verify they are independent
        assert_eq!(
            cloned_request.continuation,
            Some("original_token".to_string())
        );
        assert_eq!(cloned_request.limit, Some(30));
    }

    #[test]
    fn test_list_brand_templates_request_serialization_structure() {
        let request = ListBrandTemplatesRequest {
            continuation: Some("structure_test".to_string()),
            limit: Some(55),
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");

        // Verify JSON structure
        let json: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        assert_eq!(json["continuation"], "structure_test");
        assert_eq!(json["limit"], 55);
    }

    #[test]
    fn test_list_brand_templates_request_with_long_continuation_token() {
        let long_token = "very_long_continuation_token_".repeat(20);
        let request = ListBrandTemplatesRequest {
            continuation: Some(long_token.clone()),
            limit: Some(15),
        };

        assert_eq!(request.continuation, Some(long_token.clone()));
        assert!(long_token.len() > 500);

        // Should serialize properly even with long tokens
        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains(&long_token));
    }

    #[test]
    fn test_list_brand_templates_request_builder_pattern() {
        // Test constructing with builder-like pattern
        let mut request = ListBrandTemplatesRequest::default();
        assert!(request.continuation.is_none());
        assert!(request.limit.is_none());

        request.limit = Some(20);
        assert_eq!(request.limit, Some(20));

        request.continuation = Some("builder_token".to_string());
        assert_eq!(request.continuation, Some("builder_token".to_string()));
    }

    #[test]
    fn test_list_brand_templates_request_with_unicode() {
        let unicode_token = "令牌_🔑_τοκεν";
        let request = ListBrandTemplatesRequest {
            continuation: Some(unicode_token.to_string()),
            limit: Some(33),
        };

        assert_eq!(request.continuation, Some(unicode_token.to_string()));

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains(unicode_token));

        // Verify it can be deserialized back
        let deserialized: ListBrandTemplatesRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(deserialized.continuation, Some(unicode_token.to_string()));
        assert_eq!(deserialized.limit, Some(33));
    }

    #[test]
    fn test_list_brand_templates_request_deserialization() {
        let json = r#"{"continuation":"test_continuation","limit":77}"#;
        let request: ListBrandTemplatesRequest =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(request.continuation, Some("test_continuation".to_string()));
        assert_eq!(request.limit, Some(77));
    }

    #[test]
    fn test_list_brand_templates_request_deserialization_partial() {
        // Test deserializing with only limit
        let json_limit_only = r#"{"limit":5}"#;
        let request: ListBrandTemplatesRequest =
            serde_json::from_str(json_limit_only).expect("Failed to deserialize");

        assert!(request.continuation.is_none());
        assert_eq!(request.limit, Some(5));

        // Test deserializing with only continuation
        let json_continuation_only = r#"{"continuation":"partial_token"}"#;
        let request: ListBrandTemplatesRequest =
            serde_json::from_str(json_continuation_only).expect("Failed to deserialize");

        assert_eq!(request.continuation, Some("partial_token".to_string()));
        assert!(request.limit.is_none());

        // Test deserializing empty object
        let json_empty = r#"{}"#;
        let request: ListBrandTemplatesRequest =
            serde_json::from_str(json_empty).expect("Failed to deserialize");

        assert!(request.continuation.is_none());
        assert!(request.limit.is_none());
    }

    #[test]
    fn test_all_request_roundtrip_serialization() {
        let original_requests = vec![
            ListBrandTemplatesRequest::default(),
            ListBrandTemplatesRequest {
                continuation: None,
                limit: Some(50),
            },
            ListBrandTemplatesRequest {
                continuation: Some("roundtrip_test".to_string()),
                limit: None,
            },
            ListBrandTemplatesRequest {
                continuation: Some("full_test".to_string()),
                limit: Some(99),
            },
        ];

        for original in original_requests {
            let serialized = serde_json::to_string(&original).expect("Failed to serialize");
            let deserialized: ListBrandTemplatesRequest =
                serde_json::from_str(&serialized).expect("Failed to deserialize");

            assert_eq!(original.continuation, deserialized.continuation);
            assert_eq!(original.limit, deserialized.limit);
        }
    }
}
