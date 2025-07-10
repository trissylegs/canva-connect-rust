//! Example: Upload an asset to Canva Connect API
//!
//! This example demonstrates how to upload an image asset to the Canva Connect API.
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file
//! 3. Optionally set EXAMPLE_FILE_PATH in .env file
//! 4. Run: cargo run --example asset_upload
//!
//! Alternative: cargo run --example asset_upload -- --token YOUR_ACCESS_TOKEN --file path/to/image.png

use canva_connect::{
    auth::AccessToken,
    endpoints::assets::{AssetUploadMetadata, ListAssetsOptions},
    Client,
};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Get access token from .env file or command line arguments
    let access_token = if let Ok(token) = env::var("CANVA_ACCESS_TOKEN") {
        token
    } else {
        // Fallback to command line parsing
        let args: Vec<String> = env::args().collect();
        
        let mut access_token = None;
        let mut i = 1;
        while i < args.len() {
            if args[i] == "--token" && i + 1 < args.len() {
                access_token = Some(args[i + 1].clone());
                break;
            }
            i += 1;
        }
        
        access_token.unwrap_or_else(|| {
            eprintln!("Error: Access token not found.");
            eprintln!("Please either:");
            eprintln!("1. Set CANVA_ACCESS_TOKEN in .env file, or");
            eprintln!("2. Use: cargo run --example asset_upload -- --token YOUR_ACCESS_TOKEN --file path/to/image.png");
            std::process::exit(1);
        })
    };

    // Get file path from .env file, command line arguments, or prompt
    let file_path = if let Ok(path) = env::var("EXAMPLE_FILE_PATH") {
        path
    } else {
        // Parse command line arguments for file path
        let args: Vec<String> = env::args().collect();
        
        let mut file_path = None;
        let mut i = 1;
        while i < args.len() {
            if args[i] == "--file" && i + 1 < args.len() {
                file_path = Some(args[i + 1].clone());
                break;
            }
            i += 1;
        }
        
        file_path.unwrap_or_else(|| {
            eprintln!("Error: File path not found.");
            eprintln!("Please either:");
            eprintln!("1. Set EXAMPLE_FILE_PATH in .env file, or");
            eprintln!("2. Use: cargo run --example asset_upload -- --file path/to/image.png");
            eprintln!("3. Use: cargo run --example asset_upload -- --token YOUR_ACCESS_TOKEN --file path/to/image.png");
            std::process::exit(1);
        })
    };

    // Create the client
    let client = Client::new(AccessToken::new(access_token));
    println!("✓ Created Canva Connect API client");

    // Read the file
    let file_data = fs::read(&file_path).map_err(|e| {
        format!("Failed to read file '{}': {}", file_path, e)
    })?;
    
    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("uploaded_asset");

    println!("✓ Read file: {} ({} bytes)", file_path, file_data.len());

    // Prepare upload metadata
    let metadata = AssetUploadMetadata {
        name: file_name.to_string(),
        tags: vec!["rust-example".to_string(), "api-upload".to_string()],
    };

    println!("✓ Prepared upload metadata");

    // Create upload job
    println!("🚀 Starting asset upload...");
    let upload_job = client
        .assets()
        .create_upload_job(file_data, metadata)
        .await?;

    println!("✓ Upload job created: {}", upload_job.id);

    // Wait for the upload to complete
    println!("⏳ Waiting for upload to complete...");
    let result = client
        .assets()
        .wait_for_upload_job(&upload_job.id)
        .await?;

    println!("🎉 Upload completed successfully!");
    println!("Asset ID: {}", result.asset.id);
    println!("Asset Name: {}", result.asset.name);
    println!("Asset Type: {:?}", result.asset.asset_type);
    println!("Tags: {:?}", result.asset.tags);
    println!("Created: {}", result.asset.created_at);

    if let Some(thumbnail) = &result.asset.thumbnail {
        println!("Thumbnail: {}x{} - {}", thumbnail.width, thumbnail.height, thumbnail.url);
    }

    // List recent assets to verify the upload
    println!("\n📋 Listing recent assets...");
    let assets = client
        .assets()
        .list(Some(ListAssetsOptions {
            query: None,
            continuation: None,
            ownership: None,
            sort_by: Some(canva_connect::models::SortByType::CreatedDescending),
        }))
        .await?;

    println!("Found {} assets:", assets.items.len());
    for (i, asset) in assets.items.iter().take(5).enumerate() {
        println!("  {}. {} ({})", i + 1, asset.name, asset.id);
    }

    Ok(())
}
