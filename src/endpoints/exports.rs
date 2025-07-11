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

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::{auth::AccessToken, models::ExportQuality};

    #[test]
    fn test_exports_api_creation() {
        let access_token = AccessToken::new("test_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");

        let _exports_api = client.exports();
    }

    #[test]
    fn test_create_design_export_job_request_pdf() {
        let request = CreateDesignExportJobRequest {
            design_id: "design_123".to_string(),
            format: ExportFormat::Pdf {
                export_quality: Some(ExportQuality::Pro),
                size: None,
                pages: None,
            },
        };

        assert_eq!(request.design_id, "design_123");
        match request.format {
            ExportFormat::Pdf { export_quality, .. } => {
                assert_eq!(export_quality, Some(ExportQuality::Pro));
            }
            _ => panic!("Expected PDF format"),
        }
    }

    #[test]
    fn test_create_design_export_job_request_jpg() {
        let request = CreateDesignExportJobRequest {
            design_id: "design_456".to_string(),
            format: ExportFormat::Jpg {
                export_quality: Some(ExportQuality::Regular),
                quality: 85,
                height: Some(1080),
                width: Some(1920),
                pages: Some(vec![1, 2, 3]),
            },
        };

        assert_eq!(request.design_id, "design_456");
        match request.format {
            ExportFormat::Jpg { quality, height, width, pages, .. } => {
                assert_eq!(quality, 85);
                assert_eq!(height, Some(1080));
                assert_eq!(width, Some(1920));
                assert_eq!(pages, Some(vec![1, 2, 3]));
            }
            _ => panic!("Expected JPG format"),
        }
    }

    #[test]
    fn test_create_design_export_job_request_png() {
        let request = CreateDesignExportJobRequest {
            design_id: "design_789".to_string(),
            format: ExportFormat::Png {
                export_quality: Some(ExportQuality::Regular),
                height: None,
                width: None,
                pages: None,
            },
        };

        assert_eq!(request.design_id, "design_789");
        match request.format {
            ExportFormat::Png { export_quality, .. } => {
                assert_eq!(export_quality, Some(ExportQuality::Regular));
            }
            _ => panic!("Expected PNG format"),
        }
    }

    #[test]
    fn test_create_design_export_job_request_serialization() {
        let request = CreateDesignExportJobRequest {
            design_id: "test_design".to_string(),
            format: ExportFormat::Pdf {
                export_quality: Some(ExportQuality::Pro),
                size: None,
                pages: None,
            },
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"design_id\":\"test_design\""));
        assert!(serialized.contains("\"type\":\"pdf\""));
        assert!(serialized.contains("\"export_quality\":\"pro\""));
    }

    #[test]
    fn test_create_design_export_job_request_jpg_serialization() {
        let request = CreateDesignExportJobRequest {
            design_id: "jpg_design".to_string(),
            format: ExportFormat::Jpg {
                export_quality: Some(ExportQuality::Regular),
                quality: 90,
                height: Some(720),
                width: Some(1280),
                pages: Some(vec![1]),
            },
        };

        let serialized = serde_json::to_string(&request).expect("Failed to serialize");
        assert!(serialized.contains("\"design_id\":\"jpg_design\""));
        assert!(serialized.contains("\"type\":\"jpg\""));
        assert!(serialized.contains("\"quality\":90"));
        assert!(serialized.contains("\"height\":720"));
        assert!(serialized.contains("\"width\":1280"));
        assert!(serialized.contains("\"pages\":[1]"));
    }

    #[test]
    fn test_export_format_options_creation() {
        let options = ExportFormatOptions {
            pdf: Some(PdfExportFormatOption { available: true }),
            jpg: Some(JpgExportFormatOption { available: true }),
            png: Some(PngExportFormatOption { available: false }),
            svg: None,
            pptx: None,
            gif: None,
            mp4: None,
        };

        assert!(options.pdf.is_some());
        assert!(options.jpg.is_some());
        assert!(options.png.is_some());
        assert!(options.svg.is_none());
        assert_eq!(options.pdf.unwrap().available, true);
        assert_eq!(options.png.unwrap().available, false);
    }

    #[test]
    fn test_export_format_options_deserialization() {
        let json = r#"{"pdf":{"available":true},"jpg":{"available":false},"png":{"available":true}}"#;
        let options: ExportFormatOptions = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(options.pdf.is_some());
        assert!(options.jpg.is_some());
        assert!(options.png.is_some());
        assert_eq!(options.pdf.unwrap().available, true);
        assert_eq!(options.jpg.unwrap().available, false);
        assert_eq!(options.png.unwrap().available, true);
    }

    #[test]
    fn test_export_format_option_defaults() {
        let pdf_option = PdfExportFormatOption { available: false };
        let jpg_option = JpgExportFormatOption { available: true };
        let png_option = PngExportFormatOption { available: false };

        assert!(!pdf_option.available);
        assert!(jpg_option.available);
        assert!(!png_option.available);
    }

    #[test]
    fn test_exports_api_debug() {
        let access_token = AccessToken::new("debug_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let exports_api = client.exports();

        let debug_str = format!("{exports_api:?}");
        assert!(debug_str.contains("ExportsApi"));
    }

    #[test]
    fn test_exports_api_clone() {
        let access_token = AccessToken::new("clone_token".to_string());
        let client = Client::new(access_token).expect("Failed to create client");
        let exports_api = client.exports();

        let cloned_api = exports_api.clone();

        // Both should be separate instances but functionally equivalent
        let original_debug = format!("{exports_api:?}");
        let cloned_debug = format!("{cloned_api:?}");
        assert_eq!(original_debug, cloned_debug);
    }

    #[test]
    fn test_create_design_export_job_request_debug_format() {
        let request = CreateDesignExportJobRequest {
            design_id: "debug_design".to_string(),
            format: ExportFormat::Png {
                export_quality: Some(ExportQuality::Pro),
                height: Some(500),
                width: Some(800),
                pages: None,
            },
        };

        let debug_str = format!("{request:?}");
        assert!(debug_str.contains("CreateDesignExportJobRequest"));
        assert!(debug_str.contains("debug_design"));
    }

    #[test]
    fn test_export_format_mp4() {
        let request = CreateDesignExportJobRequest {
            design_id: "video_design".to_string(),
            format: ExportFormat::Mp4 {
                export_quality: Some(ExportQuality::Pro),
                pages: Some(vec![1, 2, 3, 4, 5]),
            },
        };

        match request.format {
            ExportFormat::Mp4 { export_quality, pages } => {
                assert_eq!(export_quality, Some(ExportQuality::Pro));
                assert_eq!(pages, Some(vec![1, 2, 3, 4, 5]));
            }
            _ => panic!("Expected MP4 format"),
        }
    }

    #[test]
    fn test_export_format_gif() {
        let request = CreateDesignExportJobRequest {
            design_id: "gif_design".to_string(),
            format: ExportFormat::Gif {
                export_quality: Some(ExportQuality::Regular),
                pages: None,
            },
        };

        match request.format {
            ExportFormat::Gif { export_quality, pages } => {
                assert_eq!(export_quality, Some(ExportQuality::Regular));
                assert_eq!(pages, None);
            }
            _ => panic!("Expected GIF format"),
        }
    }

    #[test]
    fn test_export_format_pptx() {
        let request = CreateDesignExportJobRequest {
            design_id: "presentation_design".to_string(),
            format: ExportFormat::Pptx {
                export_quality: None,
                pages: Some(vec![1, 3, 5]),
            },
        };

        match request.format {
            ExportFormat::Pptx { export_quality, pages } => {
                assert_eq!(export_quality, None);
                assert_eq!(pages, Some(vec![1, 3, 5]));
            }
            _ => panic!("Expected PPTX format"),
        }
    }

    #[test]
    fn test_all_export_format_options() {
        let options = ExportFormatOptions {
            pdf: Some(PdfExportFormatOption { available: true }),
            jpg: Some(JpgExportFormatOption { available: true }),
            png: Some(PngExportFormatOption { available: true }),
            svg: Some(SvgExportFormatOption { available: false }),
            pptx: Some(PptxExportFormatOption { available: true }),
            gif: Some(GifExportFormatOption { available: false }),
            mp4: Some(Mp4ExportFormatOption { available: true }),
        };

        // Test all format options are present
        assert!(options.pdf.is_some());
        assert!(options.jpg.is_some());
        assert!(options.png.is_some());
        assert!(options.svg.is_some());
        assert!(options.pptx.is_some());
        assert!(options.gif.is_some());
        assert!(options.mp4.is_some());

        // Test availability flags
        assert!(options.pdf.unwrap().available);
        assert!(options.jpg.unwrap().available);
        assert!(options.png.unwrap().available);
        assert!(!options.svg.unwrap().available);
        assert!(options.pptx.unwrap().available);
        assert!(!options.gif.unwrap().available);
        assert!(options.mp4.unwrap().available);
    }

    #[test]
    fn test_create_design_export_job_request_with_pages() {
        let request = CreateDesignExportJobRequest {
            design_id: "multi_page_design".to_string(),
            format: ExportFormat::Pdf {
                export_quality: Some(ExportQuality::Pro),
                size: None,
                pages: Some(vec![1, 3, 5, 7, 9]),
            },
        };

        match request.format {
            ExportFormat::Pdf { pages, .. } => {
                assert_eq!(pages, Some(vec![1, 3, 5, 7, 9]));
            }
            _ => panic!("Expected PDF format"),
        }
    }

    #[test]
    fn test_jpg_quality_bounds() {
        let request = CreateDesignExportJobRequest {
            design_id: "quality_test".to_string(),
            format: ExportFormat::Jpg {
                export_quality: None,
                quality: 100, // Maximum quality
                height: None,
                width: None,
                pages: None,
            },
        };

        match request.format {
            ExportFormat::Jpg { quality, .. } => {
                assert_eq!(quality, 100);
            }
            _ => panic!("Expected JPG format"),
        }

        let min_quality_request = CreateDesignExportJobRequest {
            design_id: "min_quality_test".to_string(),
            format: ExportFormat::Jpg {
                export_quality: None,
                quality: 1, // Minimum quality
                height: None,
                width: None,
                pages: None,
            },
        };

        match min_quality_request.format {
            ExportFormat::Jpg { quality, .. } => {
                assert_eq!(quality, 1);
            }
            _ => panic!("Expected JPG format"),
        }
    }
}
