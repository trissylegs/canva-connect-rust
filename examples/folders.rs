//! Example: Folder management with Canva Connect API
//!
//! This example demonstrates how to:
//! - Create new folders (including nested folders)
//! - Get folder details and information
//! - List items within folders
//! - Update folder names
//! - Move items between folders
//!
//! Setup:
//! 1. Copy .env.example to .env
//! 2. Set CANVA_ACCESS_TOKEN in .env file with appropriate scopes
//! 3. Run: cargo run --example folders
//!
//! (Only uses .env file for security)
//!
//! Required scopes:
//! - folder:read for getting folder details and listing items
//! - folder:write for creating, updating folders and moving items
//!
//! Note: Created folders will remain in your account after this example runs.

use canva_connect::{
    auth::AccessToken,
    endpoints::folders::{
        CreateFolderRequest, ListFolderItemsRequest, MoveFolderItemRequest, UpdateFolderRequest,
    },
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get access token from .env file
    let access_token = env::var("CANVA_ACCESS_TOKEN").unwrap_or_else(|_| {
        eprintln!("Error: CANVA_ACCESS_TOKEN not found in environment.");
        eprintln!("Please set CANVA_ACCESS_TOKEN in .env file.");
        std::process::exit(1);
    });

    // Create client
    let client = Client::new(AccessToken::new(access_token));
    let folders_api = client.folders();

    println!("📁 Canva Folders API Example\n");

    // Create a main project folder
    println!("📂 Creating a main project folder...");
    let main_folder_request = CreateFolderRequest {
        name: "API Example Project".to_string(),
        parent_folder_id: "root".to_string(), // Root level folder
    };

    let main_folder = match folders_api.create_folder(&main_folder_request).await {
        Ok(folder_response) => {
            let folder = &folder_response.folder;
            println!("✅ Created main folder:");
            println!("   ID: {}", folder.id);
            println!("   Name: {}", folder.name);
            println!("   Created: {}", folder.created_at);
            println!("   Parent: root");
            println!();
            folder.clone()
        }
        Err(e) => {
            eprintln!("❌ Failed to create main folder: {e}");
            eprintln!("   This likely means the access token doesn't have 'folder:write' scope");
            println!();
            return Ok(());
        }
    };

    // Create a subfolder within the main folder
    println!("📁 Creating a subfolder for designs...");
    let designs_folder_request = CreateFolderRequest {
        name: "Design Assets".to_string(),
        parent_folder_id: main_folder.id.clone(),
    };

    let designs_folder = match folders_api.create_folder(&designs_folder_request).await {
        Ok(folder_response) => {
            let folder = &folder_response.folder;
            println!("✅ Created designs subfolder:");
            println!("   ID: {}", folder.id);
            println!("   Name: {}", folder.name);
            println!("   Parent: {}", main_folder.id);
            println!();
            folder.clone()
        }
        Err(e) => {
            eprintln!("❌ Failed to create designs subfolder: {e}");
            println!();
            return Ok(());
        }
    };

    // Get folder details
    println!("🔍 Retrieving main folder details...");
    match folders_api.get_folder(&main_folder.id).await {
        Ok(folder_response) => {
            let folder = &folder_response.folder;
            println!("✅ Folder details:");
            println!("   ID: {}", folder.id);
            println!("   Name: {}", folder.name);
            println!("   Created: {}", folder.created_at);
            println!("   Updated: {}", folder.updated_at);
            if let Some(thumbnail) = &folder.thumbnail {
                println!("   Thumbnail: {}x{}", thumbnail.width, thumbnail.height);
            }
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to get folder details: {e}");
            eprintln!("   This likely means the access token doesn't have 'folder:read' scope");
            println!();
        }
    }

    // List items in the main folder
    println!("📋 Listing items in main folder...");
    let list_request = ListFolderItemsRequest {
        limit: Some(20),
        continuation: None,
    };

    match folders_api
        .list_folder_items(&main_folder.id, &list_request)
        .await
    {
        Ok(items_response) => {
            println!(
                "✅ Found {} item(s) in main folder:",
                items_response.items.len()
            );
            for item in &items_response.items {
                println!("   • {} ({})", item.name, item.item_type);
                println!("     ID: {}", item.id);
                if let Some(thumbnail) = &item.thumbnail {
                    println!("     Thumbnail: {}x{}", thumbnail.width, thumbnail.height);
                }
                println!("     Created: {}", item.created_at);
                println!("     Updated: {}", item.updated_at);
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list folder items: {e}");
            println!();
        }
    }

    // Update the designs folder name
    println!("✏️  Updating designs folder name...");
    let update_request = UpdateFolderRequest {
        name: "Updated Design Assets".to_string(),
    };

    match folders_api
        .update_folder(&designs_folder.id, &update_request)
        .await
    {
        Ok(folder_response) => {
            let folder = &folder_response.folder;
            println!("✅ Updated folder name:");
            println!("   Old name: Design Assets");
            println!("   New name: {}", folder.name);
            println!("   Updated: {}", folder.updated_at);
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to update folder: {e}");
            println!();
        }
    }

    // Demonstrate moving items (if any items exist)
    println!("🔄 Checking for items to move...");
    let root_list_request = ListFolderItemsRequest {
        limit: Some(10),
        continuation: None,
    };

    // Try to list items in root folder first to find something to move
    // Note: In a real scenario, you'd typically have existing designs or assets
    match folders_api
        .list_folder_items("root", &root_list_request)
        .await
    {
        Ok(root_items) => {
            if let Some(first_item) = root_items.items.first() {
                println!("📦 Found item to move: {}", first_item.name);

                let move_request = MoveFolderItemRequest {
                    item_id: first_item.id.clone(),
                    destination_folder_id: main_folder.id.clone(),
                };

                match folders_api.move_folder_item(&move_request).await {
                    Ok(()) => {
                        println!("✅ Successfully moved '{}' to main folder", first_item.name);
                        println!();
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to move item: {e}");
                        println!();
                    }
                }
            } else {
                println!("ℹ️  No items found in root folder to move");
                println!(
                    "   Create some designs or upload assets first to test moving functionality"
                );
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to list root folder items: {e}");
            eprintln!("   Note: 'root' folder access may not be available");
            println!();
        }
    }

    // Create another subfolder to demonstrate nested structure
    println!("📁 Creating another subfolder for templates...");
    let templates_folder_request = CreateFolderRequest {
        name: "Templates".to_string(),
        parent_folder_id: main_folder.id.clone(),
    };

    match folders_api.create_folder(&templates_folder_request).await {
        Ok(folder_response) => {
            let folder = &folder_response.folder;
            println!("✅ Created templates subfolder:");
            println!("   ID: {}", folder.id);
            println!("   Name: {}", folder.name);
            println!("   Parent: {}", main_folder.id);
            println!();
        }
        Err(e) => {
            eprintln!("❌ Failed to create templates subfolder: {e}");
            println!();
        }
    }

    println!("⚠️  Important Notes:");
    println!(
        "   • Created folders remain in your account (API doesn't provide delete functionality)"
    );
    println!("   • Folder hierarchy helps organize designs, assets, and other content");
    println!("   • Moving items between folders helps maintain project organization");
    println!("   • Folder names can be updated at any time");
    println!();

    println!("📋 Summary:");
    println!("   • folder:read scope allows getting folder details and listing items");
    println!("   • folder:write scope allows creating, updating folders and moving items");
    println!("   • Folders support nested hierarchies with parent-child relationships");
    println!("   • Items can be moved between folders to maintain organization");
    println!();

    println!("🎯 Folder Structure Created:");
    println!("   📁 API Example Project (main folder)");
    println!("   ├── 📁 Updated Design Assets (subfolder)");
    println!("   └── 📁 Templates (subfolder)");

    Ok(())
}
