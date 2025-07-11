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
//! Alternative: cargo run --example asset_upload -- --file path/to/image.png

use canva_connect::{auth::AccessToken, endpoints::assets::AssetUploadMetadata, Client};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable debug logging
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get access token from .env file
    let access_token = env::var("CANVA_ACCESS_TOKEN").unwrap_or_else(|_| {
        eprintln!("Error: CANVA_ACCESS_TOKEN not found in environment.");
        eprintln!("Please set CANVA_ACCESS_TOKEN in .env file.");
        std::process::exit(1);
    });

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
            std::process::exit(1);
        })
    };

    // Create the client
    let client =
        Client::new(AccessToken::new(access_token)).expect("Failed to create Canva client");
    println!("✓ Created Canva Connect API client");

    // Read the file
    let file_data =
        fs::read(&file_path).map_err(|e| format!("Failed to read file '{file_path}': {e}"))?;

    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("uploaded_asset");

    println!("✓ Read file: {} ({} bytes)", file_path, file_data.len());

    // Prepare upload metadata
    let metadata = AssetUploadMetadata::new(
        file_name,
        vec!["rust-example".to_string(), "api-upload".to_string()],
    );

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
    let asset = client.assets().wait_for_upload_job(&upload_job.id).await?;

    println!("🎉 Upload completed successfully!");
    println!("Asset ID: {}", asset.id);
    println!("Asset Name: {}", asset.name);
    println!("Asset Type: {:?}", asset.asset_type);
    println!("Tags: {:?}", asset.tags);
    println!("Created: {}", asset.created_at);

    if let Some(thumbnail) = &asset.thumbnail {
        println!(
            "Thumbnail: {}x{} - {}",
            thumbnail.width, thumbnail.height, thumbnail.url
        );
    }

    println!("\n✅ Asset upload completed successfully!");

    Ok(())
}
