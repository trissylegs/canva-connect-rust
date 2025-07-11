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
//! - How to run a local HTTP server to receive the OAuth callback
//! - How to exchange authorization code for access token using PKCE
//!
//! The example starts a local HTTP server on 127.0.0.1:8080 to automatically
//! receive the OAuth callback and complete the flow.

use canva_connect::auth::{OAuthClient, OAuthConfig, Scope};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::oneshot;
use url::Url;
use uuid::Uuid;

// HTML templates as constants to prevent indentation issues
const SUCCESS_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>OAuth Success</title>
    <meta charset="utf-8">
    <style>
        body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
        .success { color: #4CAF50; }
        .container { max-width: 600px; margin: 0 auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1 class="success">✅ Authorization Successful!</h1>
        <p>You have successfully authorized the application.</p>
        <p>The authorization code has been received and the token exchange is in progress.</p>
        <p>You can close this window and return to the terminal.</p>
    </div>
</body>
</html>"#;

const ERROR_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>OAuth Error</title>
    <meta charset="utf-8">
    <style>
        body {{ font-family: Arial, sans-serif; text-align: center; padding: 50px; }}
        .error {{ color: #f44336; }}
        .container {{ max-width: 600px; margin: 0 auto; }}
    </style>
</head>
<body>
    <div class="container">
        <h1 class="error">❌ Authorization Failed</h1>
        <p>Error: {error}</p>
        <p>Description: {error_description}</p>
        <p>Please try again or check your OAuth configuration.</p>
    </div>
</body>
</html>"#;

const INVALID_CALLBACK_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title>OAuth Error</title>
    <meta charset="utf-8">
    <style>
        body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
        .error { color: #f44336; }
        .container { max-width: 600px; margin: 0 auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1 class="error">❌ Invalid Callback</h1>
        <p>The OAuth callback is missing required parameters.</p>
        <p>Please check your OAuth configuration and try again.</p>
    </div>
</body>
</html>"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().ok();

    println!("OAuth 2.0 with PKCE Example");
    println!("===========================\n");

    // Load OAuth configuration from environment variables
    let client_id =
        std::env::var("CANVA_CLIENT_ID").unwrap_or_else(|_| "your-client-id".to_string());
    let client_secret =
        std::env::var("CANVA_CLIENT_SECRET").unwrap_or_else(|_| "your-client-secret".to_string());
    let redirect_uri = std::env::var("CANVA_REDIRECT_URI")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/callback".to_string());

    // Create OAuth configuration
    let config = OAuthConfig::new(
        client_id,
        client_secret,
        redirect_uri,
        vec![
            Scope::DesignMetaRead,
            Scope::DesignContentRead,
            Scope::AssetRead,
            Scope::ProfileRead,
        ],
    );

    let client = OAuthClient::new(config);

    // Generate a random state value for CSRF protection
    let state = Uuid::new_v4().to_string();

    // Generate authorization URL with PKCE
    let (auth_url, pkce_params) = client.authorization_url(Some(&state))?;

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

    println!("🚀 Starting local HTTP server on 127.0.0.1:8080...");
    println!("📋 Next Steps:");
    println!("1. The server is now running and waiting for OAuth callback");
    println!("2. Open the authorization URL above in your browser");
    println!("3. Grant permissions to your application");
    println!("4. You'll be redirected back and the flow will complete automatically");
    println!();

    // Channel to receive the authorization code
    let (tx, rx) = oneshot::channel();
    let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

    // Create the HTTP service
    let make_svc = make_service_fn(move |_conn| {
        let tx = tx.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let tx = tx.clone();
                handle_request(req, tx)
            }))
        }
    });

    // Start the server
    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server and wait for the OAuth callback
    println!("⏳ Waiting for OAuth callback...");
    println!("💡 If your browser doesn't open automatically, copy and paste the URL above");
    println!();

    // Try to open the URL in the default browser
    if let Err(e) = webbrowser::open(&auth_url) {
        println!("⚠️  Could not open browser automatically: {e}");
        println!("📋 Please manually open the URL above in your browser");
    }

    // Wait for the callback using select to handle both server and receiver
    let auth_code = tokio::select! {
        result = rx => {
            match result {
                Ok(code) => {
                    println!("🔄 OAuth callback received, processing...");
                    code
                }
                Err(_) => {
                    println!("❌ Failed to receive authorization code");
                    return Err("Failed to receive authorization code".into());
                }
            }
        }
        result = server => {
            if let Err(e) = result {
                println!("❌ Server error: {e}");
                return Err(format!("Server error: {e}").into());
            }
            return Err("Server terminated unexpectedly".into());
        }
    };

    println!("✅ Authorization code received!");
    println!("🔄 Exchanging authorization code for access token...");

    // Exchange code for access token using PKCE
    match client
        .exchange_code_with_pkce(&auth_code, &pkce_params)
        .await
    {
        Ok(token_response) => {
            println!("✅ Token exchange successful!");
            println!("   Access Token: {}...", &token_response.access_token[..20]);
            println!("   Token Type: {}", token_response.token_type);
            if let Some(expires_in) = token_response.expires_in {
                println!("   Expires In: {expires_in} seconds");
            }
            if let Some(refresh_token) = &token_response.refresh_token {
                println!(
                    "   Refresh Token: {}...",
                    &refresh_token[..20.min(refresh_token.len())]
                );
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

async fn handle_request(
    req: Request<Body>,
    tx: Arc<tokio::sync::Mutex<Option<oneshot::Sender<String>>>>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/callback") => {
            // Parse query parameters
            let query = req.uri().query().unwrap_or("");
            let params: HashMap<String, String> =
                Url::parse(&format!("http://example.com?{query}"))
                    .unwrap()
                    .query_pairs()
                    .into_owned()
                    .collect();

            if let Some(code) = params.get("code") {
                // Send the authorization code through the channel
                let mut tx_guard = tx.lock().await;
                if let Some(sender) = tx_guard.take() {
                    let _ = sender.send(code.clone());
                }

                // Return success page
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Body::from(SUCCESS_HTML))
                    .unwrap())
            } else if let Some(error) = params.get("error") {
                // Handle OAuth error
                let default_error = "Unknown error".to_string();
                let error_description = params.get("error_description").unwrap_or(&default_error);

                let html = ERROR_HTML_TEMPLATE
                    .replace("{error}", error)
                    .replace("{error_description}", error_description);

                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Body::from(html))
                    .unwrap())
            } else {
                // Missing required parameters
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Body::from(INVALID_CALLBACK_HTML))
                    .unwrap())
            }
        }
        _ => {
            // Handle other routes
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap())
        }
    }
}
