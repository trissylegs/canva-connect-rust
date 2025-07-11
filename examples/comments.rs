use canva_connect::{
    auth::AccessToken,
    endpoints::comments::{CreateReplyRequest, CreateThreadRequest, ListRepliesRequest},
    models::CommentThreadType,
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

    let access_token = env::var("CANVA_ACCESS_TOKEN")
        .map_err(|_| "CANVA_ACCESS_TOKEN environment variable not set")?;

    let client =
        Client::new(AccessToken::new(access_token)).expect("Failed to create Canva client");

    println!("💬 Canva Connect Comments API Example");
    println!("====================================");

    // First, we need a design to comment on
    println!("\n1. Finding a design to comment on...");
    let designs = client.designs().list(None, None, None, None).await?;

    if designs.items.is_empty() {
        println!("❌ No designs found. Create a design first to test comments.");
        return Ok(());
    }

    let design = &designs.items[0];
    let title = design.title.as_deref().unwrap_or("Untitled");
    println!("✅ Found design: {} (ID: {})", title, design.id);

    // Create a comment thread
    println!("\n2. Creating a comment thread...");
    let thread_request = CreateThreadRequest {
        message_plaintext: "This is a test comment thread created by the Rust client!".to_string(),
        assignee_id: None,
    };
    let thread_response = client
        .comments()
        .create_thread(&design.id, &thread_request)
        .await?;

    println!("✅ Comment thread created: {}", thread_response.thread.id);
    if let CommentThreadType::Comment { content, .. } = &thread_response.thread.thread_type {
        println!("   - Message: {}", content.plaintext);
    }
    println!("   - Created at: {}", thread_response.thread.created_at);

    // Get the thread details
    println!("\n3. Getting thread details...");
    let thread_details = client
        .comments()
        .get_thread(&design.id, &thread_response.thread.id)
        .await?;
    println!("✅ Thread details retrieved:");
    println!("   - ID: {}", thread_details.thread.id);
    if let CommentThreadType::Comment { content, .. } = &thread_details.thread.thread_type {
        println!("   - Message: {}", content.plaintext);
    }

    // Create a reply to the thread
    println!("\n4. Creating a reply...");
    let reply_request = CreateReplyRequest {
        message_plaintext: "This is a reply to the comment thread!".to_string(),
    };
    let reply_response = client
        .comments()
        .create_reply(&design.id, &thread_response.thread.id, &reply_request)
        .await?;

    println!("✅ Reply created: {}", reply_response.reply.id);
    println!("   - Message: {}", reply_response.reply.content.plaintext);
    println!("   - Created at: {}", reply_response.reply.created_at);

    // Get the reply details
    println!("\n5. Getting reply details...");
    let reply_details = client
        .comments()
        .get_reply(
            &design.id,
            &thread_response.thread.id,
            &reply_response.reply.id,
        )
        .await?;
    println!("✅ Reply details retrieved:");
    println!("   - ID: {}", reply_details.reply.id);
    println!("   - Message: {}", reply_details.reply.content.plaintext);

    // List all replies for the thread
    println!("\n6. Listing all replies for the thread...");
    let list_request = ListRepliesRequest::default();
    let replies = client
        .comments()
        .list_replies(&design.id, &thread_response.thread.id, &list_request)
        .await?;
    println!("✅ Found {} replies", replies.items.len());

    for (i, reply) in replies.items.iter().enumerate() {
        println!("   {}. Reply ID: {}", i + 1, reply.id);
        println!("      Message: {}", reply.content.plaintext);
        println!("      Created: {}", reply.created_at);
    }

    // Create another reply to demonstrate multiple replies
    println!("\n7. Creating another reply...");
    let second_reply_request = CreateReplyRequest {
        message_plaintext: "This is a second reply to show multiple replies working!".to_string(),
    };
    let second_reply_response = client
        .comments()
        .create_reply(
            &design.id,
            &thread_response.thread.id,
            &second_reply_request,
        )
        .await?;

    println!(
        "✅ Second reply created: {}",
        second_reply_response.reply.id
    );

    // List replies again to show the new one
    println!("\n8. Listing replies again...");
    let updated_replies = client
        .comments()
        .list_replies(&design.id, &thread_response.thread.id, &list_request)
        .await?;
    println!("✅ Now found {} replies", updated_replies.items.len());

    for (i, reply) in updated_replies.items.iter().enumerate() {
        println!("   {}. {}", i + 1, reply.content.plaintext);
    }

    println!("\n✨ Comments example completed!");
    println!(
        "ℹ️  Note: Comments cannot be deleted via the API, so these will remain on your design."
    );

    Ok(())
}
