//! Example: Design management with Canva Connect API
//!
//! This example demonstrates how to:
//! - List existing designs with various filters
//! - Create new designs (both preset and custom dimensions)
//! - Retrieve design metadata
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file with appropriate scopes
//! 3. Run: cargo run --example designs
//!
//! (Only uses .env file for security)
//!
//! Required scopes:
//! - design:meta:read for listing and getting design metadata
//! - design:content:write for creating new designs
//!
//! Note: Created designs cannot be deleted via API and will remain in your account.

use canva_connect::{
    auth::AccessToken,
    models::{CreateDesignRequest, DesignTypeInput, OwnershipType, PresetDesignTypeName},
    Client,
};
use std::env;

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

    // Create client
    let client =
        Client::new(AccessToken::new(access_token)).expect("Failed to create Canva client");
    let designs_api = client.designs();

    println!("🎨 Canva Design API Example\n");

    // List existing designs
    println!("📋 Listing your designs...");
    match designs_api
        .list(None, None, Some(OwnershipType::Owned), None)
        .await
    {
        Ok(designs_response) => {
            println!("✅ Found {} design(s):", designs_response.items.len());
            for design in &designs_response.items {
                println!(
                    "   • {} (ID: {})",
                    design.title.as_deref().unwrap_or("Untitled"),
                    design.id
                );
                if let Some(thumbnail) = &design.thumbnail {
                    println!(
                        "     Thumbnail: {}x{} - {}",
                        thumbnail.width, thumbnail.height, thumbnail.url
                    );
                }
            }
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to list designs: {e}");
            eprintln!(
                "   This likely means the access token doesn't have 'design:meta:read' scope"
            );
            println!();
        }
    }

    // Create a new presentation design
    println!("🎯 Creating a new presentation design...");
    let presentation_request = CreateDesignRequest {
        design_type: Some(DesignTypeInput::Preset {
            name: PresetDesignTypeName::Presentation,
        }),
        title: Some("Example Presentation".to_string()),
        asset_id: None,
    };

    match designs_api.create(presentation_request).await {
        Ok(created_design) => {
            let design = &created_design.design;
            println!("✅ Created presentation design:");
            println!("   ID: {}", design.id);
            println!(
                "   Title: {}",
                design.title.as_deref().unwrap_or("Untitled")
            );
            println!("   Edit URL: {}", design.urls.edit_url);
            println!("   View URL: {}", design.urls.view_url);
            if let Some(thumbnail) = &design.thumbnail {
                println!(
                    "   Thumbnail: {}x{} - {}",
                    thumbnail.width, thumbnail.height, thumbnail.url
                );
            }
            println!();

            // Get the design details
            println!("🔍 Retrieving design details...");
            match designs_api.get(&design.id).await {
                Ok(design_details) => {
                    let d = &design_details.design;
                    println!("✅ Design details:");
                    if let Some(pages) = d.page_count {
                        println!("   Pages: {pages}");
                    }
                    println!("   Owner: {} ({})", d.owner.user_id, d.owner.team_id);
                    println!(
                        "   Created: {}",
                        d.created_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!(
                        "   Updated: {}",
                        d.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!();
                }
                Err(e) => {
                    eprintln!("❌ Failed to get design details: {e}");
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create presentation: {e}");
            eprintln!(
                "   This likely means the access token doesn't have 'design:content:write' scope"
            );
            println!();
        }
    }

    // Create a custom-sized design
    println!("📐 Creating a custom-sized design (800x600)...");
    let custom_request = CreateDesignRequest {
        design_type: Some(DesignTypeInput::Custom {
            width: 800,
            height: 600,
        }),
        title: Some("Custom Design Example".to_string()),
        asset_id: None,
    };

    match designs_api.create(custom_request).await {
        Ok(created_design) => {
            let design = &created_design.design;
            println!("✅ Created custom design:");
            println!("   ID: {}", design.id);
            println!(
                "   Title: {}",
                design.title.as_deref().unwrap_or("Untitled")
            );
            println!("   Edit URL: {}", design.urls.edit_url);
            println!("   View URL: {}", design.urls.view_url);
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to create custom design: {e}");
            eprintln!(
                "   This likely means the access token doesn't have 'design:content:write' scope"
            );
            println!();
        }
    }

    println!("⚠️  Important Notes:");
    println!(
        "   • Created designs remain in your account (API doesn't provide delete functionality)"
    );
    println!("   • You can edit designs using the provided edit URLs");
    println!("   • Available design types: doc, whiteboard, presentation (preset)");
    println!("   • Custom designs can be 40-8000 pixels in width/height");
    println!();

    println!("📋 Summary:");
    println!("   • design:meta:read scope allows listing and reading design metadata");
    println!("   • design:content:write scope allows creating new designs");
    println!("   • Both preset (predefined) and custom dimensions are supported");

    Ok(())
}
