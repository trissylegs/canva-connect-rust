//! OAuth 2.0 authentication flow example with PKCE support
//!
//! This example demonstrates how to use OAuth 2.0 with PKCE (Proof Key for Code Exchange)
//! to authenticate with the Canva Connect API.
//!
//! Usage:
//!   cargo run --example oauth_flow
//!
//! This example shows:
//! - How to generate PKCE parameters
//! - How to create an authorization URL with PKCE
//! - How to exchange authorization code for access token using PKCE
//!
//! Note: This example requires manual interaction as it involves browser redirects
//! and authorization code handling.

use canva_connect::auth::{OAuthClient, OAuthConfig, Scope};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("OAuth 2.0 with PKCE Example");
    println!("===========================\n");

    // Create OAuth configuration
    let config = OAuthConfig::new(
        "your-client-id",
        "your-client-secret",
        "http://localhost:8080/callback",
        vec![
            Scope::DesignMetaRead,
            Scope::DesignContentRead,
            Scope::AssetRead,
            Scope::ProfileRead,
        ],
    );

    let client = OAuthClient::new(config);

    // Generate authorization URL with PKCE
    let (auth_url, pkce_params) = client.authorization_url(Some("random-state-value"))?;

    println!("🔐 PKCE Parameters Generated:");
    println!(
        "   Code Verifier: {} (length: {})",
        pkce_params.code_verifier,
        pkce_params.code_verifier.len()
    );
    println!("   Code Challenge: {}", pkce_params.code_challenge);
    println!();

    println!("🌐 Authorization URL:");
    println!("{auth_url}");
    println!();

    println!("📋 Next Steps:");
    println!("1. Open the authorization URL in your browser");
    println!("2. Grant permissions to your application");
    println!("3. Copy the authorization code from the redirect URL");
    println!("4. Enter the code below to exchange it for an access token");
    println!();

    // Get authorization code from user
    print!("Enter the authorization code: ");
    io::stdout().flush()?;
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim();

    if auth_code.is_empty() {
        println!("❌ No authorization code provided");
        return Ok(());
    }

    // Exchange code for access token using PKCE
    println!("\n🔄 Exchanging authorization code for access token...");
    match client
        .exchange_code_with_pkce(auth_code, Some(&pkce_params))
        .await
    {
        Ok(token_response) => {
            println!("✅ Token exchange successful!");
            println!("   Access Token: {}...", &token_response.access_token[..20]);
            println!("   Token Type: {}", token_response.token_type);
            if let Some(expires_in) = token_response.expires_in {
                println!("   Expires In: {expires_in} seconds");
            }
            if let Some(scope) = &token_response.scope {
                println!("   Granted Scopes: {scope}");
            }
        }
        Err(e) => {
            println!("❌ Token exchange failed: {e}");
        }
    }

    println!("\n🎉 OAuth 2.0 flow with PKCE completed!");
    println!("💡 The access token can now be used to authenticate API requests");

    Ok(())
}
