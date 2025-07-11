//! Autofill API endpoints for the Canva Connect API.
//!
//! This module provides access to design autofill operations including:
//! - Creating autofill jobs using brand templates
//! - Getting autofill job status and results
//! - Waiting for autofill jobs to complete
//!
//! ## Available Operations
//!
//! | Operation | Method | Endpoint | Required Scope | Description |
//! |-----------|---------|----------|----------------|-------------|
//! | [`create_autofill_job`](AutofillApi::create_autofill_job) | `POST` | `/v1/autofills` | `design:content:write` | Create a design autofill job |
//! | [`get_autofill_job`](AutofillApi::get_autofill_job) | `GET` | `/v1/autofills/{jobId}` | `design:meta:read` | Get autofill job status |
//! | [`wait_for_autofill_job`](AutofillApi::wait_for_autofill_job) | N/A | Multiple calls | `design:meta:read` | Wait for autofill job completion |
//!
//! ## OAuth Scopes
//!
//! - **`design:content:write`** - Required for creating autofill jobs
//! - **`design:meta:read`** - Required for reading autofill job status
//!
//! ## Enterprise Requirement
//!
//! The autofill API requires the user to be a member of a Canva Enterprise organization.
//!
//! ## Asynchronous Operations
//!
//! Autofill operations are asynchronous and return job IDs that can be used to check
//! the status and retrieve results. Use the `wait_for_autofill_job` method to poll
//! until completion.

use crate::{client::Client, error::Result, models::*};
use std::time::Duration;

/// Autofill API client
#[derive(Debug, Clone)]
pub struct AutofillApi {
    client: Client,
}

impl AutofillApi {
    /// Create a new autofill API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a design autofill job
    ///
    /// Starts a new asynchronous job to autofill a Canva design using a brand template and input data.
    ///
    /// # Arguments
    ///
    /// * `brand_template_id` - ID of the brand template to use for autofill
    /// * `data` - Data object containing the data fields and values to autofill
    /// * `title` - Optional title for the autofilled design
    ///
    /// # Returns
    ///
    /// Returns a `DesignAutofillJob` containing the job ID and initial status.
    ///
    /// # Errors
    ///
    /// * `Error::BadRequest` - Invalid brand template ID or data
    /// * `Error::Forbidden` - User doesn't have access to the brand template or isn't in an Enterprise organization
    /// * `Error::NotFound` - Brand template not found
    /// * `Error::RateLimitExceeded` - Rate limit exceeded (10 requests per minute per client-user)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken, models::*};
    /// use std::collections::HashMap;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"))
    ///     .expect("Failed to create client");
    /// let autofill = client.autofill();
    ///
    /// let mut data = HashMap::new();
    /// data.insert("text_field".to_string(), DatasetValue::Text {
    ///     text: "Hello, World!".to_string(),
    /// });
    /// data.insert("image_field".to_string(), DatasetValue::Image {
    ///     asset_id: "asset_123".to_string(),
    /// });
    ///
    /// let job = autofill.create_autofill_job(
    ///     "template_123",
    ///     data,
    ///     Some("My Autofilled Design".to_string())
    /// ).await?;
    ///
    /// println!("Created autofill job: {}", job.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_autofill_job(
        &self,
        brand_template_id: &str,
        data: std::collections::HashMap<String, DatasetValue>,
        title: Option<String>,
    ) -> Result<DesignAutofillJob> {
        let request = CreateDesignAutofillJobRequest {
            brand_template_id: brand_template_id.to_string(),
            title,
            data,
        };

        let response = self.client.post("/v1/autofills", &request).await?;

        let response: CreateDesignAutofillJobResponse = response.json().await?;
        Ok(response.job)
    }

    /// Get the status and result of a design autofill job
    ///
    /// Retrieves the current status of an autofill job. You might need to make multiple
    /// requests to this endpoint until you get a `success` or `failed` status.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The design autofill job ID
    ///
    /// # Returns
    ///
    /// Returns a `DesignAutofillJob` with the current status and result (if completed).
    ///
    /// # Errors
    ///
    /// * `Error::Forbidden` - User doesn't have access to the job
    /// * `Error::NotFound` - Job not found
    /// * `Error::RateLimitExceeded` - Rate limit exceeded (60 requests per minute per client-user)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken, models::*};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"))
    ///     .expect("Failed to create client");
    /// let autofill = client.autofill();
    ///
    /// let job = autofill.get_autofill_job("job_123").await?;
    ///
    /// match job.status {
    ///     DesignAutofillStatus::Success => {
    ///         if let Some(result) = job.result {
    ///             println!("Autofill completed successfully!");
    ///         }
    ///     }
    ///     DesignAutofillStatus::Failed => {
    ///         if let Some(error) = job.error {
    ///             println!("Autofill failed: {}", error.message);
    ///         }
    ///     }
    ///     DesignAutofillStatus::InProgress => {
    ///         println!("Autofill still in progress...");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_autofill_job(&self, job_id: &str) -> Result<DesignAutofillJob> {
        let response = self.client.get(&format!("/v1/autofills/{job_id}")).await?;

        let response: GetDesignAutofillJobResponse = response.json().await?;

        Ok(response.job)
    }

    /// Wait for an autofill job to complete
    ///
    /// Polls the autofill job status until it completes (success or failure).
    /// This is a convenience method that handles the polling logic.
    ///
    /// # Arguments
    ///
    /// * `job_id` - The design autofill job ID
    /// * `poll_interval` - How often to check the job status (defaults to 2 seconds)
    ///
    /// # Returns
    ///
    /// Returns a `DesignAutofillJob` with the final status and result.
    ///
    /// # Errors
    ///
    /// * `Error::Forbidden` - User doesn't have access to the job
    /// * `Error::NotFound` - Job not found
    /// * `Error::RateLimitExceeded` - Rate limit exceeded
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use canva_connect::{Client, auth::AccessToken, models::*};
    /// use std::time::Duration;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new(AccessToken::new("token"))
    ///     .expect("Failed to create client");
    /// let autofill = client.autofill();
    ///
    /// // Wait for job to complete with custom poll interval
    /// let job = autofill.wait_for_autofill_job("job_123", Some(Duration::from_secs(3))).await?;
    ///
    /// match job.status {
    ///     DesignAutofillStatus::Success => println!("Autofill completed!"),
    ///     DesignAutofillStatus::Failed => println!("Autofill failed"),
    ///     DesignAutofillStatus::InProgress => println!("Job still in progress"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn wait_for_autofill_job(
        &self,
        job_id: &str,
        poll_interval: Option<Duration>,
    ) -> Result<DesignAutofillJob> {
        let interval = poll_interval.unwrap_or(Duration::from_secs(2));

        loop {
            let job = self.get_autofill_job(job_id).await?;

            match job.status {
                DesignAutofillStatus::Success | DesignAutofillStatus::Failed => {
                    return Ok(job);
                }
                DesignAutofillStatus::InProgress => {
                    tokio::time::sleep(interval).await;
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::auth::AccessToken;
    use std::collections::HashMap;

    #[test]
    fn test_autofill_api_creation() {
        let access_token = AccessToken::new("test_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");

        let _autofill_api = client.autofill();
    }

    #[test]
    fn test_create_design_autofill_job_request_basic() {
        let mut data = HashMap::new();
        data.insert(
            "text_field".to_string(),
            DatasetValue::Text {
                text: "Hello, World!".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_123".to_string(),
            title: Some("My Design".to_string()),
            data,
        };

        assert_eq!(request.brand_template_id, "template_123");
        assert_eq!(request.title, Some("My Design".to_string()));
        assert_eq!(request.data.len(), 1);
    }

    #[test]
    fn test_create_design_autofill_job_request_without_title() {
        let mut data = HashMap::new();
        data.insert(
            "image_field".to_string(),
            DatasetValue::Image {
                asset_id: "asset_456".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_456".to_string(),
            title: None,
            data,
        };

        assert_eq!(request.brand_template_id, "template_456");
        assert!(request.title.is_none());
        assert_eq!(request.data.len(), 1);
    }

    #[test]
    fn test_create_design_autofill_job_request_with_multiple_fields() {
        let mut data = HashMap::new();
        data.insert(
            "text_field".to_string(),
            DatasetValue::Text {
                text: "Sample Text".to_string(),
            },
        );
        data.insert(
            "image_field".to_string(),
            DatasetValue::Image {
                asset_id: "asset_789".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_789".to_string(),
            title: Some("Multi-field Design".to_string()),
            data,
        };

        assert_eq!(request.brand_template_id, "template_789");
        assert_eq!(request.title, Some("Multi-field Design".to_string()));
        assert_eq!(request.data.len(), 2);
        assert!(request.data.contains_key("text_field"));
        assert!(request.data.contains_key("image_field"));
    }

    #[test]
    fn test_create_design_autofill_job_request_with_chart_data() {
        let mut data = HashMap::new();

        // Create chart data with multiple cells
        let chart_data = DataTable {
            rows: vec![
                DataTableRow {
                    cells: vec![
                        DataTableCell::String {
                            value: Some("Header 1".to_string()),
                        },
                        DataTableCell::String {
                            value: Some("Header 2".to_string()),
                        },
                    ],
                },
                DataTableRow {
                    cells: vec![
                        DataTableCell::String {
                            value: Some("Row 1 Col 1".to_string()),
                        },
                        DataTableCell::Number { value: Some(42.5) },
                    ],
                },
            ],
        };

        data.insert(
            "chart_field".to_string(),
            DatasetValue::Chart { chart_data },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_chart".to_string(),
            title: Some("Chart Design".to_string()),
            data,
        };

        assert_eq!(request.brand_template_id, "template_chart");
        assert_eq!(request.data.len(), 1);
        assert!(request.data.contains_key("chart_field"));
    }

    #[test]
    fn test_create_design_autofill_job_request_serialization() {
        let mut data = HashMap::new();
        data.insert(
            "test_field".to_string(),
            DatasetValue::Text {
                text: "Test Value".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_serialization".to_string(),
            title: Some("Serialization Test".to_string()),
            data,
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"brand_template_id\":\"template_serialization\""));
        assert!(serialized.contains("\"title\":\"Serialization Test\""));
        assert!(serialized.contains("\"test_field\""));
        assert!(serialized.contains("\"type\":\"text\""));
        assert!(serialized.contains("\"text\":\"Test Value\""));
    }

    #[test]
    fn test_create_design_autofill_job_request_serialization_no_title() {
        let mut data = HashMap::new();
        data.insert(
            "no_title_field".to_string(),
            DatasetValue::Image {
                asset_id: "asset_no_title".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "template_no_title".to_string(),
            title: None,
            data,
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"brand_template_id\":\"template_no_title\""));
        assert!(serialized.contains("\"title\":null"));
        assert!(serialized.contains("\"type\":\"image\""));
        assert!(serialized.contains("\"asset_id\":\"asset_no_title\""));
    }

    #[test]
    fn test_dataset_value_text_creation() {
        let text_value = DatasetValue::Text {
            text: "Hello World".to_string(),
        };

        match text_value {
            DatasetValue::Text { text } => {
                assert_eq!(text, "Hello World");
            }
            _ => panic!("Expected text dataset value"),
        }
    }

    #[test]
    fn test_dataset_value_image_creation() {
        let image_value = DatasetValue::Image {
            asset_id: "asset_123".to_string(),
        };

        match image_value {
            DatasetValue::Image { asset_id } => {
                assert_eq!(asset_id, "asset_123");
            }
            _ => panic!("Expected image dataset value"),
        }
    }

    #[test]
    fn test_dataset_value_chart_creation() {
        let chart_data = DataTable {
            rows: vec![DataTableRow {
                cells: vec![DataTableCell::String {
                    value: Some("Test".to_string()),
                }],
            }],
        };

        let chart_value = DatasetValue::Chart {
            chart_data: chart_data.clone(),
        };

        match chart_value {
            DatasetValue::Chart { chart_data: data } => {
                assert_eq!(data.rows.len(), 1);
                assert_eq!(data.rows[0].cells.len(), 1);
            }
            _ => panic!("Expected chart dataset value"),
        }
    }

    #[test]
    fn test_data_table_cell_types() {
        let string_cell = DataTableCell::String {
            value: Some("test".to_string()),
        };
        let number_cell = DataTableCell::Number { value: Some(3.14) };
        let boolean_cell = DataTableCell::Boolean { value: Some(true) };
        let date_cell = DataTableCell::Date {
            value: Some(1640995200), // 2022-01-01 00:00:00 UTC
        };

        // Test string cell
        match string_cell {
            DataTableCell::String { value } => {
                assert_eq!(value, Some("test".to_string()));
            }
            _ => panic!("Expected string cell"),
        }

        // Test number cell
        match number_cell {
            DataTableCell::Number { value } => {
                assert_eq!(value, Some(3.14));
            }
            _ => panic!("Expected number cell"),
        }

        // Test boolean cell
        match boolean_cell {
            DataTableCell::Boolean { value } => {
                assert_eq!(value, Some(true));
            }
            _ => panic!("Expected boolean cell"),
        }

        // Test date cell
        match date_cell {
            DataTableCell::Date { value } => {
                assert_eq!(value, Some(1640995200));
            }
            _ => panic!("Expected date cell"),
        }
    }

    #[test]
    fn test_data_table_cell_types_with_none_values() {
        let string_cell = DataTableCell::String { value: None };
        let number_cell = DataTableCell::Number { value: None };
        let boolean_cell = DataTableCell::Boolean { value: None };
        let date_cell = DataTableCell::Date { value: None };

        // All should handle None values properly
        match string_cell {
            DataTableCell::String { value } => assert!(value.is_none()),
            _ => panic!("Expected string cell"),
        }

        match number_cell {
            DataTableCell::Number { value } => assert!(value.is_none()),
            _ => panic!("Expected number cell"),
        }

        match boolean_cell {
            DataTableCell::Boolean { value } => assert!(value.is_none()),
            _ => panic!("Expected boolean cell"),
        }

        match date_cell {
            DataTableCell::Date { value } => assert!(value.is_none()),
            _ => panic!("Expected date cell"),
        }
    }

    #[test]
    fn test_autofill_api_debug() {
        let access_token = AccessToken::new("debug_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let autofill_api = client.autofill();

        let debug_str = format!("{autofill_api:?}");
        assert!(debug_str.contains("AutofillApi"));
    }

    #[test]
    fn test_autofill_api_clone() {
        let access_token = AccessToken::new("clone_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let autofill_api = client.autofill();

        let cloned_api = autofill_api.clone();

        // Both should be separate instances but functionally equivalent
        let original_debug = format!("{autofill_api:?}");
        let cloned_debug = format!("{cloned_api:?}");
        assert_eq!(original_debug, cloned_debug);
    }

    #[test]
    fn test_create_design_autofill_job_request_debug_format() {
        let mut data = HashMap::new();
        data.insert(
            "debug_field".to_string(),
            DatasetValue::Text {
                text: "Debug Text".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "debug_template".to_string(),
            title: Some("Debug Title".to_string()),
            data,
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("CreateDesignAutofillJobRequest"));
        assert!(debug_str.contains("debug_template"));
        assert!(debug_str.contains("Debug Title"));
        assert!(debug_str.contains("debug_field"));
    }

    #[test]
    fn test_create_design_autofill_job_request_with_unicode() {
        let mut data = HashMap::new();
        data.insert(
            "unicode_field".to_string(),
            DatasetValue::Text {
                text: "🎨 Unicode Text 测试 τεκστ".to_string(),
            },
        );

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "unicode_template_🌍".to_string(),
            title: Some("Unicode Title 📝".to_string()),
            data,
        };

        assert_eq!(request.brand_template_id, "unicode_template_🌍");
        assert_eq!(request.title, Some("Unicode Title 📝".to_string()));

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("🎨 Unicode Text 测试 τεκστ"));
        assert!(serialized.contains("unicode_template_🌍"));
        assert!(serialized.contains("Unicode Title 📝"));
    }

    #[test]
    fn test_create_design_autofill_job_request_with_empty_data() {
        let data = HashMap::new();

        let request = CreateDesignAutofillJobRequest {
            brand_template_id: "empty_data_template".to_string(),
            title: Some("Empty Data Test".to_string()),
            data,
        };

        assert_eq!(request.brand_template_id, "empty_data_template");
        assert_eq!(request.data.len(), 0);

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"data\":{}"));
    }

    #[test]
    fn test_dataset_value_serialization() {
        let text_value = DatasetValue::Text {
            text: "Serialization Test".to_string(),
        };
        let image_value = DatasetValue::Image {
            asset_id: "serialization_asset".to_string(),
        };

        let text_serialized = serde_json::to_string(&text_value).expect("Failed to serialize text");
        let image_serialized =
            serde_json::to_string(&image_value).expect("Failed to serialize image");

        assert!(text_serialized.contains("\"type\":\"text\""));
        assert!(text_serialized.contains("\"text\":\"Serialization Test\""));

        assert!(image_serialized.contains("\"type\":\"image\""));
        assert!(image_serialized.contains("\"asset_id\":\"serialization_asset\""));
    }

    #[test]
    fn test_complex_chart_data_structure() {
        let chart_data = DataTable {
            rows: vec![
                // Header row
                DataTableRow {
                    cells: vec![
                        DataTableCell::String {
                            value: Some("Month".to_string()),
                        },
                        DataTableCell::String {
                            value: Some("Sales".to_string()),
                        },
                        DataTableCell::String {
                            value: Some("Active".to_string()),
                        },
                    ],
                },
                // Data rows
                DataTableRow {
                    cells: vec![
                        DataTableCell::String {
                            value: Some("January".to_string()),
                        },
                        DataTableCell::Number {
                            value: Some(1500.75),
                        },
                        DataTableCell::Boolean { value: Some(true) },
                    ],
                },
                DataTableRow {
                    cells: vec![
                        DataTableCell::String {
                            value: Some("February".to_string()),
                        },
                        DataTableCell::Number {
                            value: Some(2300.50),
                        },
                        DataTableCell::Boolean { value: Some(false) },
                    ],
                },
            ],
        };

        assert_eq!(chart_data.rows.len(), 3);
        assert_eq!(chart_data.rows[0].cells.len(), 3); // Header
        assert_eq!(chart_data.rows[1].cells.len(), 3); // January
        assert_eq!(chart_data.rows[2].cells.len(), 3); // February

        let serialized = serde_json::to_string(&chart_data).expect("Failed to serialize");
        assert!(serialized.contains("Month"));
        assert!(serialized.contains("1500.75"));
        assert!(serialized.contains("January"));
    }

    #[test]
    fn test_create_design_autofill_job_request_roundtrip_serialization() {
        let mut data = HashMap::new();
        data.insert(
            "roundtrip_text".to_string(),
            DatasetValue::Text {
                text: "Roundtrip Test".to_string(),
            },
        );
        data.insert(
            "roundtrip_image".to_string(),
            DatasetValue::Image {
                asset_id: "roundtrip_asset".to_string(),
            },
        );

        let original_request = CreateDesignAutofillJobRequest {
            brand_template_id: "roundtrip_template".to_string(),
            title: Some("Roundtrip Title".to_string()),
            data,
        };

        let serialized = serde_json::to_string(&original_request).expect("Failed to serialize");
        let deserialized: CreateDesignAutofillJobRequest =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(
            original_request.brand_template_id,
            deserialized.brand_template_id
        );
        assert_eq!(original_request.title, deserialized.title);
        assert_eq!(original_request.data.len(), deserialized.data.len());

        // Check that the data fields are preserved
        assert!(deserialized.data.contains_key("roundtrip_text"));
        assert!(deserialized.data.contains_key("roundtrip_image"));
    }
}
