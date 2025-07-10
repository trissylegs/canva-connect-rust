//! Example: Get user profile information from Canva Connect API
//!
//! This example demonstrates how to retrieve user profile information including
//! basic identification, profile details, and capabilities.
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file with appropriate scopes
//! 3. Run: cargo run --example user_profile
//!
//! Alternative: cargo run --example user_profile -- --token YOUR_ACCESS_TOKEN
//!
//! Required scopes for full functionality:
//! - No scope required for basic user identification
//! - profile:read for display name and capabilities

use canva_connect::{auth::AccessToken, Client};
use std::env;

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
            eprintln!("2. Use: cargo run --example user_profile -- --token YOUR_ACCESS_TOKEN");
            std::process::exit(1);
        })
    };

    // Create client
    let client = Client::new(AccessToken::new(access_token));
    let user_api = client.user();

    println!("🔍 Fetching user information...\n");

    // Get basic user identification (no scope required)
    match user_api.get_me().await {
        Ok(me) => {
            println!("✅ Basic User Information:");
            println!("   User ID: {}", me.user_id);
            println!("   Team ID: {}", me.team_id);
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to get basic user info: {e}");
            println!();
        }
    }

    // Get user profile (requires profile:read scope)
    match user_api.get_profile().await {
        Ok(profile) => {
            println!("✅ User Profile:");
            println!("   Display Name: {}", profile.display_name);
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to get user profile: {e}");
            eprintln!("   This likely means the access token doesn't have 'profile:read' scope");
            println!();
        }
    }

    // Get user capabilities (requires profile:read scope)
    match user_api.get_capabilities().await {
        Ok(capabilities) => {
            println!("✅ User Capabilities:");
            if capabilities.is_empty() {
                println!("   No premium capabilities available");
                println!("   Note: Capabilities like 'autofill' and 'brand_template' require Canva Enterprise");
                println!("         'resize' capability requires Canva Pro or higher");
            } else {
                for capability in capabilities {
                    match capability {
                        canva_connect::endpoints::user::Capability::Autofill => {
                            println!("   🚀 Autofill - Access to autofill APIs (Canva Enterprise)");
                        }
                        canva_connect::endpoints::user::Capability::BrandTemplate => {
                            println!("   🎨 Brand Templates - Access to brand template APIs (Canva Enterprise)");
                        }
                        canva_connect::endpoints::user::Capability::Resize => {
                            println!("   📐 Resize - Access to design resize APIs (Canva Pro+)");
                        }
                    }
                }
            }
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to get user capabilities: {e}");
            eprintln!("   This likely means the access token doesn't have 'profile:read' scope");
            println!();
        }
    }

    println!("📋 Summary:");
    println!("   • Basic user info (user ID, team ID) requires no specific scope");
    println!("   • Profile and capabilities require 'profile:read' scope");
    println!("   • Capabilities determine which advanced APIs you can access");

    Ok(())
}
