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
