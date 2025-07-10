//! Integration tests for Canva Connect API client
//!
//! These tests require real API credentials and make actual HTTP requests to the Canva Connect API.
//! They are designed to be safe and non-destructive, but should be run with caution.
//!
//! ## Setup
//!
//! To run integration tests, set the following environment variables:
//! - `CANVA_ACCESS_TOKEN` - Valid Canva Connect API access token
//! - `CANVA_INTEGRATION_TESTS=1` - Enable integration tests
//!
//! ## Running Integration Tests
//!
//! ```bash
//! # Run only integration tests
//! CANVA_INTEGRATION_TESTS=1 cargo test --test integration
//!
//! # Run all tests including integration tests
//! CANVA_INTEGRATION_TESTS=1 cargo test
//! ```
//!
//! ## Safety
//!
//! - Tests only read data or create temporary assets that are cleaned up
//! - Tests respect rate limits with built-in delays
//! - Tests will skip if credentials are not available

use std::env;
use std::sync::Once;
use tokio::time::{sleep, Duration};

use canva_connect::{auth::AccessToken, Client};

static INIT: Once = Once::new();

/// Test configuration loaded from environment variables
pub struct TestConfig {
    pub access_token: AccessToken,
    pub client: Client,
}

impl TestConfig {
    /// Create test configuration from environment variables
    /// Returns None if integration tests are disabled or credentials are missing
    pub fn from_env() -> Option<Self> {
        // Check if integration tests are enabled
        if env::var("CANVA_INTEGRATION_TESTS").unwrap_or_default() != "1" {
            println!("Integration tests disabled. Set CANVA_INTEGRATION_TESTS=1 to enable.");
            return None;
        }

        // Get access token
        let token_str = env::var("CANVA_ACCESS_TOKEN").ok()?;
        let access_token = AccessToken::new(token_str);
        let client = Client::new(access_token.clone());

        Some(TestConfig {
            access_token,
            client,
        })
    }

    /// Initialize test environment (call once per test run)
    pub fn init() {
        INIT.call_once(|| {
            // Load .env file if it exists
            dotenv::dotenv().ok();

            // Initialize logging for tests
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .try_init()
                .ok();
        });
    }
}

/// Helper to respect rate limits between API calls
pub async fn rate_limit_delay() {
    sleep(Duration::from_millis(500)).await;
}

/// Skip test if integration tests are not configured
macro_rules! skip_if_no_config {
    () => {{
        TestConfig::init();
        match TestConfig::from_env() {
            Some(config) => config,
            None => {
                println!("Skipping integration test - no configuration available");
                return;
            }
        }
    }};
}

/// Common test utilities
pub mod utils {
    use super::*;
    use std::collections::HashSet;

    /// Track assets created during tests for cleanup
    pub struct AssetTracker {
        created_assets: HashSet<String>,
    }

    impl Default for AssetTracker {
        fn default() -> Self {
            Self::new()
        }
    }

    impl AssetTracker {
        pub fn new() -> Self {
            Self {
                created_assets: HashSet::new(),
            }
        }

        pub fn track_asset(&mut self, asset_id: String) {
            self.created_assets.insert(asset_id);
        }

        /// Clean up all tracked assets
        pub async fn cleanup(&self, client: &Client) {
            for asset_id in &self.created_assets {
                if let Err(e) = client.assets().delete(asset_id).await {
                    eprintln!("Failed to delete test asset {asset_id}: {e}");
                }
                rate_limit_delay().await;
            }
        }
    }

    impl Drop for AssetTracker {
        fn drop(&mut self) {
            if !self.created_assets.is_empty() {
                eprintln!(
                    "Warning: {} test assets may not have been cleaned up",
                    self.created_assets.len()
                );
            }
        }
    }
}

// User API integration tests
#[tokio::test]
async fn test_get_user_profile() {
    let config = skip_if_no_config!();

    let user_api = config.client.user();

    // Test getting user profile
    let profile = user_api
        .get_profile()
        .await
        .expect("Failed to get user profile");

    // Verify profile structure
    assert!(
        !profile.display_name.is_empty(),
        "Display name should not be empty"
    );

    println!("✅ User profile: {}", profile.display_name);

    rate_limit_delay().await;
}

#[tokio::test]
async fn test_get_user_capabilities() {
    let config = skip_if_no_config!();

    let user_api = config.client.user();

    // Test getting user capabilities
    let capabilities = user_api
        .get_capabilities()
        .await
        .expect("Failed to get user capabilities");

    // Capabilities can be empty for basic users, but the call should succeed
    println!("✅ User capabilities: {capabilities:?}");

    // Verify each capability is valid
    for capability in &capabilities {
        match capability {
            canva_connect::endpoints::user::Capability::Autofill => {
                println!("  - Has autofill capability");
            }
            canva_connect::endpoints::user::Capability::BrandTemplate => {
                println!("  - Has brand template capability");
            }
            canva_connect::endpoints::user::Capability::Resize => {
                println!("  - Has resize capability");
            }
        }
    }

    rate_limit_delay().await;
}

// Assets API integration tests
use canva_connect::endpoints::assets::{CreateUrlAssetUploadJobRequest, UpdateAssetRequest};

#[tokio::test]
async fn test_url_asset_upload_workflow() {
    let config = skip_if_no_config!();
    let mut tracker = utils::AssetTracker::new();

    let assets_api = config.client.assets();

    // Test URL upload with a small, reliable test image
    let upload_request = CreateUrlAssetUploadJobRequest {
        url: "https://rustacean.net/assets/rustacean-flat-happy.png".to_string(),
        name: "Integration Test Rustacean".to_string(),
    };

    println!("📤 Starting URL upload test...");

    // Create upload job
    let job = assets_api
        .create_url_upload_job(upload_request)
        .await
        .expect("Failed to create URL upload job");

    println!("✅ Upload job created: {}", job.id);
    assert!(!job.id.is_empty());

    rate_limit_delay().await;

    // Poll for completion (with timeout)
    println!("⏳ Waiting for upload completion...");
    let mut attempts = 0;
    let max_attempts = 30; // 30 seconds max

    let asset = loop {
        attempts += 1;

        let current_job = assets_api
            .get_url_upload_job(&job.id)
            .await
            .expect("Failed to get upload job status");

        match current_job.status {
            canva_connect::models::JobStatus::Success => {
                let asset = current_job
                    .asset
                    .expect("Job succeeded but no asset returned");
                println!("✅ Upload completed successfully!");
                break asset;
            }
            canva_connect::models::JobStatus::Failed => {
                let error_msg = current_job
                    .error
                    .map(|e| format!("{}: {}", e.code, e.message))
                    .unwrap_or_else(|| "Unknown error".to_string());
                panic!("Upload failed: {error_msg}");
            }
            canva_connect::models::JobStatus::InProgress => {
                if attempts >= max_attempts {
                    panic!("Upload timed out after {max_attempts} seconds");
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        }
    };

    // Track asset for cleanup
    tracker.track_asset(asset.id.clone());

    // Verify asset properties
    assert!(!asset.id.is_empty());
    assert!(!asset.name.is_empty());
    assert!(!asset.created_at.to_string().is_empty());

    println!("✅ Asset created: {} ({})", asset.id, asset.name);

    rate_limit_delay().await;

    // Test getting the asset
    let retrieved_asset = assets_api
        .get(&asset.id)
        .await
        .expect("Failed to retrieve uploaded asset");

    assert_eq!(asset.id, retrieved_asset.id);
    assert_eq!(asset.name, retrieved_asset.name);

    println!("✅ Asset retrieval verified");

    rate_limit_delay().await;

    // Test updating the asset
    let update_request = UpdateAssetRequest {
        name: Some("Updated Integration Test Asset".to_string()),
        tags: Some(vec!["integration-test".to_string(), "rust".to_string()]),
    };

    let updated_asset = assets_api
        .update(&asset.id, update_request)
        .await
        .expect("Failed to update asset");

    assert_eq!(updated_asset.id, asset.id);
    assert_eq!(updated_asset.name, "Updated Integration Test Asset");
    assert!(updated_asset.tags.contains(&"integration-test".to_string()));
    assert!(updated_asset.tags.contains(&"rust".to_string()));

    println!("✅ Asset update verified");

    rate_limit_delay().await;

    // Clean up the test asset
    tracker.cleanup(&config.client).await;

    println!("✅ URL asset upload workflow test completed");
}

#[tokio::test]
async fn test_asset_error_handling() {
    let config = skip_if_no_config!();

    let assets_api = config.client.assets();

    // Test getting non-existent asset
    let result = assets_api.get("non-existent-asset-id").await;
    assert!(result.is_err());

    match result {
        Err(canva_connect::Error::Api { code, message }) => {
            println!("✅ Correct error for non-existent asset: {code} - {message}");
        }
        _ => panic!("Expected API error for non-existent asset"),
    }

    rate_limit_delay().await;

    println!("✅ Asset error handling test completed");
}

// Designs API integration tests
use canva_connect::models::{
    CreateDesignRequest, CustomDesignTypeInput, DesignTypeInput, OwnershipType,
    PresetDesignTypeInput, PresetDesignTypeName, SortByType,
};

#[tokio::test]
async fn test_list_designs() {
    let config = skip_if_no_config!();

    let designs_api = config.client.designs();

    // Test listing designs without filters
    let designs = designs_api
        .list(None, None, None, None)
        .await
        .expect("Failed to list designs");

    println!("✅ Listed {} designs", designs.items.len());

    // Verify structure
    for design in &designs.items {
        assert!(!design.id.is_empty());
        assert!(!design.urls.edit_url.is_empty());
        assert!(!design.urls.view_url.is_empty());
        assert!(!design.owner.user_id.is_empty());
        assert!(!design.owner.team_id.is_empty());
    }

    rate_limit_delay().await;

    // Test with filters
    let filtered_designs = designs_api
        .list(
            None,
            None,
            Some(OwnershipType::Any),
            Some(SortByType::ModifiedDescending),
        )
        .await
        .expect("Failed to list filtered designs");

    println!(
        "✅ Listed {} filtered designs",
        filtered_designs.items.len()
    );

    rate_limit_delay().await;
}

#[tokio::test]
async fn test_create_and_get_design() {
    let config = skip_if_no_config!();

    let designs_api = config.client.designs();

    // Create a new presentation design
    let create_request = CreateDesignRequest {
        design_type: Some(DesignTypeInput::Preset(PresetDesignTypeInput {
            design_type: PresetDesignTypeName::Presentation,
        })),
        title: Some("Integration Test Presentation".to_string()),
        asset_id: None,
    };

    println!("📄 Creating new presentation design...");

    let created_design = designs_api
        .create(create_request)
        .await
        .expect("Failed to create design");

    println!("✅ Created design: {}", created_design.design.id);
    assert!(!created_design.design.id.is_empty());
    assert_eq!(
        created_design.design.title,
        Some("Integration Test Presentation".to_string())
    );

    rate_limit_delay().await;

    // Get the created design
    let retrieved_design = designs_api
        .get(&created_design.design.id)
        .await
        .expect("Failed to get design");

    assert_eq!(retrieved_design.design.id, created_design.design.id);
    assert_eq!(retrieved_design.design.title, created_design.design.title);
    assert!(!retrieved_design.design.urls.edit_url.is_empty());
    assert!(!retrieved_design.design.urls.view_url.is_empty());

    println!("✅ Retrieved design verified");
    println!("   Title: {:?}", retrieved_design.design.title);
    println!("   Pages: {:?}", retrieved_design.design.page_count);
    println!("   Edit URL: {}", retrieved_design.design.urls.edit_url);

    rate_limit_delay().await;

    println!("✅ Create and get design test completed");
}

#[tokio::test]
async fn test_create_custom_design() {
    let config = skip_if_no_config!();

    let designs_api = config.client.designs();

    // Create a custom-sized design
    let create_request = CreateDesignRequest {
        design_type: Some(DesignTypeInput::Custom(CustomDesignTypeInput {
            width: 800,
            height: 600,
        })),
        title: Some("Custom Integration Test Design".to_string()),
        asset_id: None,
    };

    println!("📄 Creating custom design (800x600)...");

    let created_design = designs_api
        .create(create_request)
        .await
        .expect("Failed to create custom design");

    println!("✅ Created custom design: {}", created_design.design.id);
    assert!(!created_design.design.id.is_empty());
    assert_eq!(
        created_design.design.title,
        Some("Custom Integration Test Design".to_string())
    );

    rate_limit_delay().await;

    // Verify we can get the custom design
    let retrieved_design = designs_api
        .get(&created_design.design.id)
        .await
        .expect("Failed to get custom design");

    assert_eq!(retrieved_design.design.id, created_design.design.id);
    println!("✅ Custom design verified");

    rate_limit_delay().await;

    println!("✅ Custom design test completed");
}

#[tokio::test]
async fn test_design_error_handling() {
    let config = skip_if_no_config!();

    let designs_api = config.client.designs();

    // Test getting non-existent design
    let result = designs_api.get("non-existent-design-id").await;
    assert!(result.is_err());

    match result {
        Err(canva_connect::Error::Api { code, message }) => {
            println!("✅ Correct error for non-existent design: {code} - {message}");
        }
        _ => panic!("Expected API error for non-existent design"),
    }

    rate_limit_delay().await;

    println!("✅ Design error handling test completed");
}
