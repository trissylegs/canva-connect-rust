//! Exports API endpoints
//!
//! This module provides access to the Canva Exports API, allowing you to
//! export designs to various file formats.

use crate::{
    client::Client,
    error::Result,
    models::{ExportFormat, ExportJob},
};
use serde::{Deserialize, Serialize};

/// Client for the Exports API
#[derive(Debug, Clone)]
pub struct ExportsApi {
    client: Client,
}

/// Request to create a design export job
#[derive(Debug, Clone, Serialize)]
pub struct CreateDesignExportJobRequest {
    /// The design ID to export
    pub design_id: String,
    /// The export format
    pub format: ExportFormat,
}

/// Response from creating a design export job
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDesignExportJobResponse {
    /// The export job
    pub job: ExportJob,
}

/// Response from getting a design export job
#[derive(Debug, Clone, Deserialize)]
pub struct GetDesignExportJobResponse {
    /// The export job
    pub job: ExportJob,
}

/// Response from getting available export formats
#[derive(Debug, Clone, Deserialize)]
pub struct GetDesignExportFormatsResponse {
    /// Available formats for the design
    pub formats: ExportFormatOptions,
}

/// Available export format options
#[derive(Debug, Clone, Deserialize)]
pub struct ExportFormatOptions {
    /// PDF export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<PdfExportFormatOption>,
    /// JPG export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpg: Option<JpgExportFormatOption>,
    /// PNG export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png: Option<PngExportFormatOption>,
    /// SVG export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<SvgExportFormatOption>,
    /// PPTX export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pptx: Option<PptxExportFormatOption>,
    /// GIF export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif: Option<GifExportFormatOption>,
    /// MP4 export option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp4: Option<Mp4ExportFormatOption>,
}

/// PDF export format option
#[derive(Debug, Clone, Deserialize)]
pub struct PdfExportFormatOption {
    /// Whether PDF export is available
    #[serde(default)]
    pub available: bool,
}

/// JPG export format option
#[derive(Debug, Clone, Deserialize)]
pub struct JpgExportFormatOption {
    /// Whether JPG export is available
    #[serde(default)]
    pub available: bool,
}

/// PNG export format option
#[derive(Debug, Clone, Deserialize)]
pub struct PngExportFormatOption {
    /// Whether PNG export is available
    #[serde(default)]
    pub available: bool,
}

/// SVG export format option
#[derive(Debug, Clone, Deserialize)]
pub struct SvgExportFormatOption {
    /// Whether SVG export is available
    #[serde(default)]
    pub available: bool,
}

/// PPTX export format option
#[derive(Debug, Clone, Deserialize)]
pub struct PptxExportFormatOption {
    /// Whether PPTX export is available
    #[serde(default)]
    pub available: bool,
}

/// GIF export format option
#[derive(Debug, Clone, Deserialize)]
pub struct GifExportFormatOption {
    /// Whether GIF export is available
    #[serde(default)]
    pub available: bool,
}

/// MP4 export format option
#[derive(Debug, Clone, Deserialize)]
pub struct Mp4ExportFormatOption {
    /// Whether MP4 export is available
    #[serde(default)]
    pub available: bool,
}

impl ExportsApi {
    /// Create a new exports API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a design export job
    ///
    /// Starts a new asynchronous job to export a design to a file format.
    ///
    /// **Required OAuth scope:** `design:content:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn create_design_export_job(
        &self,
        request: &CreateDesignExportJobRequest,
    ) -> Result<CreateDesignExportJobResponse> {
        let response = self.client.post("/v1/exports", request).await?;
        Ok(response.json::<CreateDesignExportJobResponse>().await?)
    }

    /// Get a design export job
    ///
    /// Gets the result of a design export job.
    ///
    /// **Required OAuth scope:** `design:content:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_design_export_job(
        &self,
        export_id: &str,
    ) -> Result<GetDesignExportJobResponse> {
        let url = format!("/v1/exports/{export_id}");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetDesignExportJobResponse>().await?)
    }

    /// Get available export formats for a design
    ///
    /// Lists the available file formats for exporting a design.
    ///
    /// **Required OAuth scope:** `design:content:read`
    #[cfg_attr(feature = "observability", tracing::instrument(skip(self)))]
    pub async fn get_design_export_formats(
        &self,
        design_id: &str,
    ) -> Result<GetDesignExportFormatsResponse> {
        let url = format!("/v1/designs/{design_id}/export-formats");
        let response = self.client.get(&url).await?;
        Ok(response.json::<GetDesignExportFormatsResponse>().await?)
    }
}
