//! OAuth 2.0 Authorization Code Flow Example
//!
//! This example demonstrates how to implement the complete OAuth 2.0 authorization
//! code flow with the Canva Connect API.
//!
//! ## Setup
//!
//! 1. Create a Canva app at https://developers.canva.com/
//! 2. Set up your environment variables:
//!    ```bash
//!    export CANVA_CLIENT_ID=your_client_id
//!    export CANVA_CLIENT_SECRET=your_client_secret
//!    export CANVA_REDIRECT_URI=http://localhost:8080/callback
//!    ```
//! 3. Run the example:
//!    ```bash
//!    cargo run --example oauth2_flow
//!    ```
//!
//! ## Flow Overview
//!
//! 1. Start a local HTTP server to handle the OAuth callback
//! 2. Generate and display the authorization URL
//! 3. User visits the URL and authorizes the application
//! 4. Canva redirects back to our callback URL with an authorization code
//! 5. Exchange the authorization code for an access token
//! 6. Use the access token to make API requests
//!
//! ## Security Notes
//!
//! - Never expose your client secret in client-side code
//! - Use HTTPS in production environments
//! - Implement proper state parameter validation to prevent CSRF attacks
//! - Store tokens securely and implement token refresh if needed

use canva_connect::auth::{OAuthClient, OAuthConfig, Scope};
use canva_connect::Client;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Shared state for the OAuth flow
#[derive(Debug, Clone, Default)]
struct OAuthState {
    authorization_code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

/// Parse query parameters from a URL
fn parse_query_params(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|param| {
            let mut parts = param.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((
                    key.to_string(),
                    urlencoding::decode(value).unwrap_or_default().to_string(),
                )),
                _ => None,
            }
        })
        .collect()
}

/// Start a local HTTP server to handle the OAuth callback
async fn start_callback_server(
    port: u16,
    oauth_state: Arc<Mutex<OAuthState>>,
    expected_state: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("🔗 Callback server listening on http://localhost:{port}/callback");

    loop {
        let (mut stream, _) = listener.accept().await?;
        let oauth_state_clone = Arc::clone(&oauth_state);
        let expected_state_clone = expected_state.clone();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            if let Ok(n) = tokio::io::AsyncReadExt::read(&mut stream, &mut buffer).await {
                let request = String::from_utf8_lossy(&buffer[..n]);

                // Parse the HTTP request line
                if let Some(first_line) = request.lines().next() {
                    if let Some(path) = first_line.split_whitespace().nth(1) {
                        if path.starts_with("/callback") {
                            // Handle OAuth callback
                            let mut state = oauth_state_clone.lock().await;

                            if let Some(query_start) = path.find('?') {
                                let query = &path[query_start + 1..];
                                let params = parse_query_params(query);

                                // Check for errors
                                if let Some(error) = params.get("error") {
                                    state.error = Some(error.clone());
                                    let response = create_error_response(error);
                                    let _ = tokio::io::AsyncWriteExt::write_all(
                                        &mut stream,
                                        response.as_bytes(),
                                    )
                                    .await;
                                    return;
                                }

                                // Validate state parameter
                                if let Some(returned_state) = params.get("state") {
                                    if returned_state != &expected_state_clone {
                                        state.error = Some("Invalid state parameter".to_string());
                                        let response =
                                            create_error_response("Invalid state parameter");
                                        let _ = tokio::io::AsyncWriteExt::write_all(
                                            &mut stream,
                                            response.as_bytes(),
                                        )
                                        .await;
                                        return;
                                    }
                                    state.state = Some(returned_state.clone());
                                }

                                // Extract authorization code
                                if let Some(code) = params.get("code") {
                                    state.authorization_code = Some(code.clone());
                                    let response = create_success_response();
                                    let _ = tokio::io::AsyncWriteExt::write_all(
                                        &mut stream,
                                        response.as_bytes(),
                                    )
                                    .await;
                                } else {
                                    state.error =
                                        Some("No authorization code received".to_string());
                                    let response =
                                        create_error_response("No authorization code received");
                                    let _ = tokio::io::AsyncWriteExt::write_all(
                                        &mut stream,
                                        response.as_bytes(),
                                    )
                                    .await;
                                }
                            } else {
                                // No query parameters
                                state.error = Some("No query parameters received".to_string());
                                let response =
                                    create_error_response("No query parameters received");
                                let _ = tokio::io::AsyncWriteExt::write_all(
                                    &mut stream,
                                    response.as_bytes(),
                                )
                                .await;
                            }
                        } else {
                            // Not our callback path
                            let response = create_not_found_response();
                            let _ = tokio::io::AsyncWriteExt::write_all(
                                &mut stream,
                                response.as_bytes(),
                            )
                            .await;
                        }
                    }
                }
            }
        });

        // Check if we've received the authorization code
        {
            let state = oauth_state.lock().await;
            if state.authorization_code.is_some() || state.error.is_some() {
                break;
            }
        }
    }

    Ok(())
}

/// Create an HTTP success response
fn create_success_response() -> String {
    "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\
         Connection: close\r\n\r\n\
         <html><body>\
         <h1>✅ Authorization Successful!</h1>\
         <p>You can close this window and return to the terminal.</p>\
         </body></html>"
        .to_string()
}

/// Create an HTTP error response
fn create_error_response(error: &str) -> String {
    format!(
        "HTTP/1.1 400 Bad Request\r\n\
         Content-Type: text/html\r\n\
         Connection: close\r\n\r\n\
         <html><body>\
         <h1>❌ Authorization Failed</h1>\
         <p>Error: {error}</p>\
         <p>Please try again.</p>\
         </body></html>"
    )
}

/// Create an HTTP not found response
fn create_not_found_response() -> String {
    "HTTP/1.1 404 Not Found\r\n\
     Content-Type: text/html\r\n\
     Connection: close\r\n\r\n\
     <html><body>\
     <h1>404 Not Found</h1>\
     <p>The requested path was not found.</p>\
     </body></html>"
        .to_string()
}

/// Load configuration from environment variables
fn load_config() -> Result<OAuthConfig, Box<dyn std::error::Error>> {
    let client_id =
        env::var("CANVA_CLIENT_ID").map_err(|_| "CANVA_CLIENT_ID environment variable not set")?;

    let client_secret = env::var("CANVA_CLIENT_SECRET")
        .map_err(|_| "CANVA_CLIENT_SECRET environment variable not set")?;

    let redirect_uri = env::var("CANVA_REDIRECT_URI")
        .unwrap_or_else(|_| "http://localhost:8080/callback".to_string());

    // Request comprehensive scopes for demonstration
    let scopes = vec![
        Scope::AssetRead,
        Scope::AssetWrite,
        Scope::DesignMetaRead,
        Scope::DesignContentRead,
        Scope::DesignContentWrite,
        Scope::BrandTemplateMetaRead,
        Scope::BrandTemplateContentRead,
        Scope::FolderRead,
        Scope::FolderWrite,
        Scope::CommentRead,
        Scope::CommentWrite,
        Scope::ProfileRead,
    ];

    Ok(OAuthConfig::new(
        client_id,
        client_secret,
        redirect_uri,
        scopes,
    ))
}

/// Extract port from redirect URI
fn extract_port_from_redirect_uri(redirect_uri: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let url = url::Url::parse(redirect_uri)?;
    match url.port() {
        Some(port) => Ok(port),
        None => match url.scheme() {
            "http" => Ok(80),
            "https" => Ok(443),
            _ => Err("Unable to determine port from redirect URI".into()),
        },
    }
}

/// Demonstrate API usage with the obtained access token
async fn demonstrate_api_usage(access_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 Testing API access with the obtained token...");

    let client = Client::new(access_token.into())?;

    // Get user identity
    match client.user().get_me().await {
        Ok(user_summary) => {
            println!("✅ Successfully retrieved user identity:");
            println!("   • User ID: {}", user_summary.user_id);
            println!("   • Team ID: {}", user_summary.team_id);
        }
        Err(e) => {
            println!("❌ Failed to retrieve user identity: {e}");
        }
    }

    // Get user profile
    match client.user().get_profile().await {
        Ok(profile) => {
            println!("✅ Successfully retrieved user profile:");
            println!("   • Display Name: {}", profile.display_name);
        }
        Err(e) => {
            println!("❌ Failed to retrieve user profile: {e}");
        }
    }

    // Get user capabilities
    match client.user().get_capabilities().await {
        Ok(capabilities) => {
            println!("✅ Successfully retrieved user capabilities:");
            if capabilities.is_empty() {
                println!("   • No special capabilities (standard Canva user)");
            } else {
                for capability in &capabilities {
                    match capability {
                        canva_connect::endpoints::user::Capability::Autofill => {
                            println!("   • Can use autofill APIs (Enterprise feature)");
                        }
                        canva_connect::endpoints::user::Capability::BrandTemplate => {
                            println!("   • Can use brand template APIs (Enterprise feature)");
                        }
                        canva_connect::endpoints::user::Capability::Resize => {
                            println!("   • Can create design resize jobs (Pro feature)");
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to retrieve user capabilities: {e}");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Canva Connect OAuth 2.0 Authorization Code Flow Example");
    println!("==============================================================");

    // Load configuration
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Configuration Error: {e}");
            eprintln!("\n💡 Please set up your environment variables:");
            eprintln!("   export CANVA_CLIENT_ID=your_client_id");
            eprintln!("   export CANVA_CLIENT_SECRET=your_client_secret");
            eprintln!("   export CANVA_REDIRECT_URI=http://localhost:8080/callback  # optional");
            eprintln!("\n📚 Get your credentials at: https://developers.canva.com/");
            return Err(e);
        }
    };

    // Extract port from redirect URI
    let port = extract_port_from_redirect_uri(&config.redirect_uri)?;

    // Create OAuth client
    let oauth_client = OAuthClient::new(config);

    // Generate state parameter for CSRF protection
    let state = Uuid::new_v4().to_string();

    // Generate authorization URL
    let auth_url = oauth_client.authorization_url(Some(&state))?;

    println!("\n📋 OAuth Configuration:");
    println!("   • Client ID: {}", oauth_client.config().client_id);
    println!("   • Redirect URI: {}", oauth_client.config().redirect_uri);
    println!("   • Scopes: {}", oauth_client.config().scopes_string());
    println!("   • State: {state}");

    // Start callback server
    let oauth_state = Arc::new(Mutex::new(OAuthState::default()));
    let server_state = Arc::clone(&oauth_state);
    let server_state_param = state.clone();

    tokio::spawn(async move {
        if let Err(e) = start_callback_server(port, server_state, server_state_param).await {
            eprintln!("❌ Callback server error: {e}");
        }
    });

    // Give server time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n🌐 Authorization URL:");
    println!("{auth_url}");
    println!("\n📝 Instructions:");
    println!("1. Copy the URL above and paste it into your browser");
    println!("2. Log in to Canva and authorize the application");
    println!("3. You'll be redirected back to this application");
    println!("4. Return to this terminal to see the results");

    // Wait for user to open the URL
    print!("\n⏳ Press Enter after you've opened the URL in your browser...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("⏳ Waiting for authorization callback...");

    // Wait for callback with timeout
    let timeout = tokio::time::Duration::from_secs(300); // 5 minutes
    let start = tokio::time::Instant::now();

    loop {
        {
            let check_state = Arc::clone(&oauth_state);
            let state = check_state.lock().await;
            if let Some(error) = &state.error {
                eprintln!("❌ Authorization failed: {error}");
                return Err(error.clone().into());
            }

            if let Some(code) = &state.authorization_code {
                println!("✅ Authorization successful! Received authorization code.");

                // Exchange code for access token
                println!("🔄 Exchanging authorization code for access token...");
                match oauth_client.exchange_code(code).await {
                    Ok(token_response) => {
                        println!("✅ Token exchange successful!");
                        println!(
                            "   • Access token: {}...{}",
                            &token_response.access_token[..8],
                            &token_response.access_token[token_response.access_token.len() - 8..]
                        );
                        println!("   • Token type: {}", token_response.token_type);
                        if let Some(expires_in) = token_response.expires_in {
                            println!("   • Expires in: {expires_in} seconds");
                        }
                        if let Some(scope) = &token_response.scope {
                            println!("   • Granted scopes: {scope}");
                        }

                        // Demonstrate API usage
                        if let Err(e) = demonstrate_api_usage(&token_response.access_token).await {
                            eprintln!("⚠️  API demonstration failed: {e}");
                        }

                        println!("\n🎉 OAuth 2.0 flow completed successfully!");
                        println!("💾 You can now use the access token to make API requests:");
                        println!(
                            "   let client = Client::new(\"{}\".into());",
                            token_response.access_token
                        );

                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("❌ Token exchange failed: {e}");
                        return Err(e.into());
                    }
                }
            }
        }

        // Check timeout
        if start.elapsed() > timeout {
            eprintln!(
                "❌ Authorization timed out after {} seconds",
                timeout.as_secs()
            );
            return Err("Authorization timeout".into());
        }

        // Short delay before checking again
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
