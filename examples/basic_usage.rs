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
    endpoints::assets::{AssetUploadMetadata, CreateUrlAssetUploadJobRequest, ListAssetsOptions},
    models::SortByType,
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
        upload_metadata: AssetUploadMetadata {
            name: "Sample Image from Unsplash".to_string(),
            tags: vec!["rust-example".to_string(), "url-upload".to_string()],
        },
    };

    let upload_job = client
        .assets()
        .create_url_upload_job(url_upload_request)
        .await?;

    println!("✓ URL upload job created: {}", upload_job.id);

    // Wait for the upload to complete
    println!("⏳ Waiting for upload to complete...");
    let result = client
        .assets()
        .wait_for_url_upload_job(&upload_job.id)
        .await?;

    println!("🎉 Upload completed successfully!");
    println!("Asset ID: {}", result.asset.id);
    println!("Asset Name: {}", result.asset.name);
    println!("Asset Type: {:?}", result.asset.asset_type);

    // Example 2: List assets
    println!("\n📋 Listing your assets...");
    
    let assets = client
        .assets()
        .list(Some(ListAssetsOptions {
            query: None,
            continuation: None,
            ownership: None,
            sort_by: Some(SortByType::CreatedDescending),
        }))
        .await?;

    println!("Found {} assets:", assets.items.len());
    for (i, asset) in assets.items.iter().take(10).enumerate() {
        println!("  {}. {} ({})", i + 1, asset.name, asset.id);
        if let Some(thumbnail) = &asset.thumbnail {
            println!("     Thumbnail: {}x{}", thumbnail.width, thumbnail.height);
        }
    }

    // Example 3: Get specific asset details
    if let Some(first_asset) = assets.items.first() {
        println!("\n🔍 Getting details for first asset...");
        
        let asset_details = client
            .assets()
            .get(&first_asset.id)
            .await?;

        println!("Asset Details:");
        println!("  Name: {}", asset_details.name);
        println!("  Type: {:?}", asset_details.asset_type);
        println!("  Tags: {:?}", asset_details.tags);
        println!("  Created: {}", asset_details.created_at);
        println!("  Updated: {}", asset_details.updated_at);
    }

    // Example 4: Search assets
    println!("\n🔍 Searching for assets with 'rust' tag...");
    
    let search_results = client
        .assets()
        .list(Some(ListAssetsOptions {
            query: Some("rust".to_string()),
            continuation: None,
            ownership: None,
            sort_by: Some(SortByType::CreatedDescending),
        }))
        .await?;

    println!("Found {} assets matching 'rust':", search_results.items.len());
    for (i, asset) in search_results.items.iter().take(5).enumerate() {
        println!("  {}. {} ({})", i + 1, asset.name, asset.id);
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
