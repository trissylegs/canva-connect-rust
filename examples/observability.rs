//! Example: Using OpenTelemetry observability with Canva Connect API
//!
//! This example demonstrates how to enable distributed tracing for API calls
//! using OpenTelemetry. The traces can be exported to Jaeger, OTEL Collector,
//! or any other OTLP-compatible backend.
//!
//! Setup:
//! 1. Start a local Jaeger instance:
//!    docker run -d -p 14268:14268 -p 16686:16686 -p 4317:4317 -p 4318:4318 \
//!    jaegertracing/all-in-one:latest --collector.otlp.enabled=true
//! 2. Copy .env.example to .env and set CANVA_ACCESS_TOKEN
//! 3. Run with observability feature: cargo run --example observability --features observability
//! 4. Open Jaeger UI at http://localhost:16686 to view traces
//!
//! Alternative OTLP endpoints:
//! - Local OTEL Collector: http://localhost:4317
//! - Honeycomb: https://api.honeycomb.io:443 (requires API key)
//! - Other vendors: check their OTLP endpoint documentation

use canva_connect::{auth::AccessToken, Client};
use std::env;

#[cfg(feature = "observability")]
use canva_connect::observability::init_tracing;
#[cfg(feature = "observability")]
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing (only works with observability feature)
    #[cfg(feature = "observability")]
    let _guard = {
        let otlp_endpoint =
            env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());

        info!("Initializing tracing with endpoint: {}", otlp_endpoint);
        init_tracing("canva-connect-example", &otlp_endpoint).await?
    };

    #[cfg(not(feature = "observability"))]
    {
        println!("⚠️  Observability feature not enabled!");
        println!("   Run with: cargo run --example observability --features observability");
        println!("   This will add distributed tracing to all API calls.");
        println!();
    }

    // Get access token
    let access_token = env::var("CANVA_ACCESS_TOKEN")
        .map_err(|_| "CANVA_ACCESS_TOKEN environment variable not set")?;

    let client = Client::new(AccessToken::new(access_token));

    #[cfg(feature = "observability")]
    info!("Starting Canva Connect API demonstration");

    println!("🔍 Demonstrating traced API calls...");
    println!();

    // These API calls will be automatically traced when observability is enabled
    demonstrate_user_apis(&client).await?;
    demonstrate_asset_apis(&client).await?;

    #[cfg(feature = "observability")]
    info!("API demonstration completed");

    println!("✅ All API calls completed!");
    println!();

    #[cfg(feature = "observability")]
    {
        println!("📊 Observability Information:");
        println!("   • All API calls have been traced with OpenTelemetry");
        println!("   • HTTP requests include spans with method, URL, status code");
        println!("   • Asset operations include file size and name metadata");
        println!("   • Check your tracing backend for detailed trace data");
        println!("   • Jaeger UI: http://localhost:16686 (if using local Jaeger)");
    }

    #[cfg(not(feature = "observability"))]
    {
        println!("💡 To enable observability:");
        println!("   1. Rebuild with: cargo build --features observability");
        println!("   2. Start Jaeger: docker run -d -p 16686:16686 -p 4317:4317 jaegertracing/all-in-one --collector.otlp.enabled=true");
        println!("   3. Run this example again to see distributed tracing in action");
    }

    Ok(())
}

async fn demonstrate_user_apis(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "observability")]
    info!("Demonstrating User API with tracing");

    println!("👤 User API Operations:");

    // Get basic user info - this will create a span named "get_me"
    match client.user().get_me().await {
        Ok(me) => {
            println!(
                "   ✅ Retrieved user identification: {} (team: {})",
                me.user_id, me.team_id
            );
        }
        Err(e) => {
            #[cfg(feature = "observability")]
            warn!("Failed to get user info: {}", e);
            println!("   ❌ Failed to get user info: {e}");
        }
    }

    // Get user profile - this will create a span named "get_profile"
    match client.user().get_profile().await {
        Ok(profile) => {
            println!("   ✅ Retrieved user profile: {}", profile.display_name);
        }
        Err(e) => {
            #[cfg(feature = "observability")]
            warn!("Failed to get user profile: {}", e);
            println!("   ❌ Failed to get user profile: {e}");
        }
    }

    // Get user capabilities - traced automatically
    match client.user().get_capabilities().await {
        Ok(capabilities) => {
            println!("   ✅ Retrieved {} user capabilities", capabilities.len());
        }
        Err(e) => {
            #[cfg(feature = "observability")]
            warn!("Failed to get user capabilities: {}", e);
            println!("   ❌ Failed to get user capabilities: {e}");
        }
    }

    println!();
    Ok(())
}

async fn demonstrate_asset_apis(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "observability")]
    info!("Demonstrating Asset API with tracing");

    println!("🖼️  Asset API Operations:");

    // Try to get a sample asset (this will likely fail, but will be traced)
    let sample_asset_id = "BAD12345"; // Intentionally invalid ID for demo

    match client.assets().get(sample_asset_id).await {
        Ok(asset) => {
            println!("   ✅ Retrieved asset: {}", asset.id);
        }
        Err(e) => {
            #[cfg(feature = "observability")]
            warn!("Expected failure getting sample asset: {}", e);
            println!("   ❌ Expected failure getting sample asset: {e}");
            println!("      (This is normal - we used an invalid asset ID for demonstration)");
        }
    }

    println!("   💡 The above API calls were traced and include:");
    println!("      • HTTP method and URL");
    println!("      • Response status codes");
    println!("      • Request duration");
    println!("      • Custom metadata (asset names, file sizes, etc.)");

    println!();
    Ok(())
}
