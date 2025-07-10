//! Example: Basic usage of the Canva Connect API client
//!
//! This example demonstrates basic operations with the Canva Connect API:
//! - Creating a client
//! - Uploading an asset from a URL
//! - Listing assets
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file
//! 3. Run: cargo run --example basic_usage
//!
//! Alternative: cargo run --example basic_usage -- --token YOUR_ACCESS_TOKEN

use canva_connect::{
    auth::AccessToken,
    endpoints::assets::{AssetUploadMetadata, CreateUrlAssetUploadJobRequest},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get access token from .env file or command line arguments
    let access_token = if let Ok(token) = env::var("CANVA_ACCESS_TOKEN") {
        token
    } else {
        // Fallback to command line arguments
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 && args[1] == "--token" {
            args[2].clone()
        } else {
            eprintln!("Error: Access token not found.");
            eprintln!("Please either:");
            eprintln!("1. Set CANVA_ACCESS_TOKEN in .env file, or");
            eprintln!("2. Use: cargo run --example basic_usage -- --token YOUR_ACCESS_TOKEN");
            std::process::exit(1);
        }
    };

    // Create the client
    let client = Client::new(AccessToken::new(access_token));
    println!("✓ Created Canva Connect API client");

    // Example 1: Upload an asset from URL
    println!("\n🚀 Uploading asset from URL...");

    let url_upload_request = CreateUrlAssetUploadJobRequest {
        url: "https://images.unsplash.com/photo-1518837695005-2083093ee35b".to_string(),
        upload_metadata: AssetUploadMetadata::new(
            "Sample Image from Unsplash",
            vec!["rust-example".to_string(), "url-upload".to_string()],
        ),
    };

    let upload_job = client
        .assets()
        .create_url_upload_job(url_upload_request)
        .await?;

    println!("✓ URL upload job created: {}", upload_job.id);

    // Wait for the upload to complete
    println!("⏳ Waiting for upload to complete...");
    let asset = client
        .assets()
        .wait_for_url_upload_job(&upload_job.id)
        .await?;

    println!("🎉 Upload completed successfully!");
    println!("Asset ID: {}", asset.id);
    println!("Asset Name: {}", asset.name);
    println!("Asset Type: {:?}", asset.asset_type);

    println!("\n✅ Basic usage example completed successfully!");

    Ok(())
}
