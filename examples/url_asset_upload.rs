//! Example: Upload an asset from a URL to Canva Connect API
//!
//! This example demonstrates how to upload an asset to Canva from a publicly
//! accessible URL. The process involves creating a URL upload job and polling
//! for completion.
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file with appropriate scopes
//! 3. Run: cargo run --example url_asset_upload -- --url "https://rustacean.net/assets/rustacean-flat-happy.png"
//!
//! Alternative: Set IMAGE_URL environment variable
//!
//! Required scopes:
//! - asset:write (to create upload jobs)
//! - asset:read (to check upload status)
//!
//! Supported file formats:
//! - Images: JPEG, PNG, GIF, BMP, TIFF, SVG, WebP
//! - Videos: MP4, MOV, AVI, WebM
//! - Audio: MP3, WAV, AAC, OGG
//! - Documents: PDF

use canva_connect::{
    auth::AccessToken,
    endpoints::assets::{CreateUrlAssetUploadJobRequest, UpdateAssetRequest},
    Client,
};
use std::{env, io::Write};

#[cfg(feature = "observability")]
use canva_connect::observability::init_tracing;
#[cfg(feature = "observability")]
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable debug logging
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize tracing if observability is enabled
    #[cfg(feature = "observability")]
    let _guard = {
        let otlp_endpoint =
            env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());
        info!("Initializing tracing for URL asset upload");
        init_tracing("canva-url-upload-example", &otlp_endpoint).await?
    };

    // Get access token from environment or fail
    let access_token = env::var("CANVA_ACCESS_TOKEN")
        .map_err(|_| "CANVA_ACCESS_TOKEN environment variable not set")?;

    // Get URL from command line arguments or environment
    let image_url = get_image_url_from_args_or_env()?;

    println!("🌐 Canva Connect API - URL Asset Upload Example");
    println!("================================================");
    println!();
    println!("📋 Upload Details:");
    println!("   Source URL: {image_url}");
    println!();

    // Create client
    let client =
        Client::new(AccessToken::new(access_token)).expect("Failed to create Canva client");
    let assets_api = client.assets();

    #[cfg(feature = "observability")]
    info!("Starting URL asset upload process");

    // Extract filename from URL
    let filename = extract_filename_from_url(&image_url);

    println!("🚀 Creating URL upload job...");

    // Create URL upload job
    let request = CreateUrlAssetUploadJobRequest {
        url: image_url.clone(),
        name: filename,
    };

    let job = match assets_api.create_url_upload_job(request).await {
        Ok(job) => {
            println!("   ✅ Upload job created successfully!");
            println!("   📋 Job ID: {}", job.id);
            println!("   📊 Status: {:?}", job.status);
            println!();
            job
        }
        Err(e) => {
            eprintln!("   ❌ Failed to create upload job: {e}");
            eprintln!();
            eprintln!("💡 Common issues:");
            eprintln!("   • URL is not publicly accessible");
            eprintln!("   • File format not supported by Canva");
            eprintln!("   • File size exceeds limits");
            eprintln!("   • Missing required OAuth scopes (asset:write)");
            return Err(e.into());
        }
    };

    println!("⏳ Waiting for upload to complete...");
    println!("   (This may take a few seconds depending on file size)");
    println!();

    // Wait for job completion with progress updates
    let asset = match wait_for_url_upload_completion(&assets_api, &job.id).await {
        Ok(asset) => {
            println!("   ✅ Upload completed successfully!");
            asset
        }
        Err(e) => {
            eprintln!("   ❌ Upload failed: {e}");
            eprintln!();
            eprintln!("💡 This might be due to:");
            eprintln!("   • URL became inaccessible during upload");
            eprintln!("   • File format not supported");
            eprintln!("   • Network timeout or connectivity issues");
            eprintln!("   • File corrupted or invalid");
            return Err(e.into());
        }
    };

    // Display initial asset information
    println!("🎉 Asset Upload Complete!");
    println!("========================");
    println!();
    println!("📄 Initial Asset Details:");
    println!("   Asset ID: {}", asset.id);
    println!("   Name: {}", asset.name);
    println!("   Tags: {:?}", asset.tags);
    println!("   Created: {}", asset.created_at);
    println!("   Updated: {}", asset.updated_at);

    if let Some(thumbnail) = &asset.thumbnail {
        println!(
            "   Thumbnail: {}x{} pixels",
            thumbnail.width, thumbnail.height
        );
        println!("   Thumbnail URL: {}", thumbnail.url);
    }

    // Update the asset with a better name and tags
    println!();
    println!("🔄 Updating asset with better metadata...");

    let update_request = UpdateAssetRequest {
        name: Some("🦀 Happy Rustacean Mascot".to_string()),
        tags: Some(vec![
            "rust".to_string(),
            "mascot".to_string(),
            "rustacean".to_string(),
            "api-upload".to_string(),
            "url-source".to_string(),
        ]),
    };

    let updated_asset = match assets_api.update(&asset.id, update_request).await {
        Ok(updated) => {
            println!("   ✅ Asset updated successfully!");
            updated
        }
        Err(e) => {
            eprintln!("   ❌ Failed to update asset: {e}");
            eprintln!("   ℹ️  Continuing with original asset...");
            asset
        }
    };

    // Display updated asset information
    println!();
    println!("📄 Updated Asset Details:");
    println!("   Asset ID: {}", updated_asset.id);
    println!("   Name: {}", updated_asset.name);
    println!("   Tags: {:?}", updated_asset.tags);
    println!("   Created: {}", updated_asset.created_at);
    println!("   Updated: {}", updated_asset.updated_at);

    println!();
    println!("🔗 You can now use this asset in Canva designs!");
    println!(
        "   Asset ID '{}' can be referenced in other API calls",
        updated_asset.id
    );

    #[cfg(feature = "observability")]
    info!("URL asset upload completed successfully");

    #[cfg(feature = "observability")]
    {
        println!();
        println!("📊 Observability Info:");
        println!("   • This upload was traced with OpenTelemetry");
        println!("   • Check your tracing backend for detailed metrics");
        println!("   • Request IDs are captured for support correlation");
    }

    Ok(())
}

/// Extract command line arguments or environment variable for image URL
fn get_image_url_from_args_or_env() -> Result<String, Box<dyn std::error::Error>> {
    // First try environment variable
    if let Ok(url) = env::var("IMAGE_URL") {
        return Ok(url);
    }

    // Then try command line arguments
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--url" && i + 1 < args.len() {
            return Ok(args[i + 1].clone());
        }
        i += 1;
    }

    // Show help if no URL provided
    eprintln!("❌ No image URL provided!");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  cargo run --example url_asset_upload -- --url \"https://rustacean.net/assets/rustacean-flat-happy.png\"");
    eprintln!();
    eprintln!("Or set environment variable:");
    eprintln!("  IMAGE_URL=\"https://rustacean.net/assets/rustacean-flat-happy.png\"");
    eprintln!();
    eprintln!("Example Rustacean URLs to try:");
    eprintln!("  https://rustacean.net/assets/rustacean-flat-happy.png");
    eprintln!("  https://rustacean.net/assets/rustacean-flat-gesture.png");
    eprintln!("  https://rustacean.net/assets/cuddlyferris.png");

    Err("No image URL provided".into())
}

/// Extract filename from URL, with fallback
fn extract_filename_from_url(url: &str) -> String {
    // Try to extract filename from URL path
    if let Ok(parsed_url) = url::Url::parse(url) {
        if let Some(mut segments) = parsed_url.path_segments() {
            if let Some(last_segment) = segments.next_back() {
                if !last_segment.is_empty() && last_segment.contains('.') {
                    return last_segment.to_string();
                }
            }
        }
    }

    // Fallback to generic name with timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("url-upload-{timestamp}.jpg")
}

/// Wait for URL upload completion with progress updates
async fn wait_for_url_upload_completion(
    assets_api: &canva_connect::endpoints::assets::AssetsApi,
    job_id: &str,
) -> Result<canva_connect::models::Asset, canva_connect::Error> {
    let mut attempt = 0;
    let max_attempts = 30; // 60 seconds total with 2-second intervals

    loop {
        attempt += 1;

        #[cfg(feature = "observability")]
        tracing::debug!("Checking upload job status, attempt {}", attempt);

        let job = assets_api.get_url_upload_job(job_id).await?;

        match job.status {
            canva_connect::models::JobStatus::Success => {
                return job.asset.ok_or_else(|| {
                    canva_connect::Error::Generic("Job succeeded but no asset returned".to_string())
                });
            }
            canva_connect::models::JobStatus::Failed => {
                let error_msg = job
                    .error
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .unwrap_or_else(|| "Unknown upload error".to_string());

                #[cfg(feature = "observability")]
                tracing::error!("URL upload job failed: {}", error_msg);

                return Err(canva_connect::Error::Generic(format!(
                    "Upload failed: {error_msg}"
                )));
            }
            canva_connect::models::JobStatus::InProgress => {
                // Show progress indicator
                let dots = ".".repeat((attempt % 4) + 1);
                print!("\r   ⏳ Processing{dots:<4}");
                std::io::stdout().flush().ok();

                if attempt >= max_attempts {
                    #[cfg(feature = "observability")]
                    tracing::warn!("URL upload job timed out after {} attempts", max_attempts);

                    return Err(canva_connect::Error::Generic(
                        "Upload timed out - job is taking longer than expected".to_string(),
                    ));
                }

                // Wait before polling again
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}
