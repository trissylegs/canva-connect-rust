use canva_connect::endpoints::brand_templates::ListBrandTemplatesRequest;
use canva_connect::models::{
    DataField, DatasetValue, DesignAutofillJobResult, DesignAutofillStatus,
};
use canva_connect::{auth::AccessToken, Client};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

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

    println!("🎨 Canva Connect Autofill API Example");
    println!("=====================================");

    // First, let's list some brand templates to find one we can autofill
    println!("\n1. Listing brand templates...");
    let templates = client
        .brand_templates()
        .list(&ListBrandTemplatesRequest::default())
        .await?;

    if templates.items.is_empty() {
        println!("❌ No brand templates found. You need brand templates to use autofill.");
        return Ok(());
    }

    // Search through multiple templates to find one with a dataset
    println!("\n2. Searching for templates with datasets...");
    let mut found_template = None;
    for template in templates.items.iter().take(10) {
        // Try first 10 templates
        println!(
            "   Checking template: {} (ID: {})",
            template.title, template.id
        );
        match client.brand_templates().get_dataset(&template.id).await {
            Ok(dataset) => {
                println!(
                    "     ✅ Found dataset with {} fields",
                    dataset.dataset.len()
                );
                // Check if has text fields
                for (field_name, field_info) in &dataset.dataset {
                    if matches!(field_info, DataField::Text { .. }) {
                        println!("     ✅ Found text field: {field_name}");
                        found_template = Some(template);
                        break;
                    }
                }
                if found_template.is_some() {
                    break;
                }
            }
            Err(_) => {
                println!("     ❌ No dataset available");
                continue;
            }
        }
    }

    let template = match found_template {
        Some(template) => template,
        None => {
            println!("❌ No brand templates found with datasets for autofill");
            return Ok(());
        }
    };

    println!(
        "✅ Using template: {} (ID: {})",
        template.title, template.id
    );

    // Get the template details to see what data we can autofill
    println!("\n3. Getting template details...");
    let template_details = client.brand_templates().get(&template.id).await?;
    println!("✅ Template details retrieved");
    println!("   - Name: {}", template_details.brand_template.title);

    // Get dataset to see what fields are available
    println!("\n4. Getting dataset fields...");
    let dataset = client.brand_templates().get_dataset(&template.id).await?;
    println!("✅ Dataset retrieved with {} fields", dataset.dataset.len());

    // Use actual field names from dataset
    let mut data = HashMap::new();
    for (field_name, field_info) in dataset.dataset {
        match field_info {
            DataField::Text { .. } => {
                data.insert(
                    field_name.clone(),
                    DatasetValue::Text {
                        text: format!("Sample text for {field_name}"),
                    },
                );
                println!("   - Adding text field: {field_name}");
                if data.len() >= 2 {
                    break;
                } // Only fill first 2 text fields
            }
            _ => {
                println!("   - Skipping non-text field: {field_name}");
            }
        }
    }

    if data.is_empty() {
        println!("❌ Template has no text fields to autofill");
        return Ok(());
    }

    // Now create autofill job with real field names
    println!("\n5. Creating autofill job with {} fields...", data.len());
    let autofill_job = client
        .autofill()
        .create_autofill_job(&template.id, data, None)
        .await?;

    println!("✅ Autofill job created: {}", autofill_job.id);

    // Wait for the job to complete
    println!("\n6. Waiting for autofill job to complete...");
    let mut attempts = 0;
    let max_attempts = 30;

    loop {
        let job_status = client.autofill().get_autofill_job(&autofill_job.id).await?;

        println!("   Job status: {:?}", job_status.status);

        match job_status.status {
            DesignAutofillStatus::Success => {
                println!("✅ Autofill completed successfully!");
                if let Some(DesignAutofillJobResult::CreateDesign { design }) = &job_status.result {
                    println!("   Created design: {}", design.id);
                }
                break;
            }
            DesignAutofillStatus::Failed => {
                println!("❌ Autofill job failed");
                if let Some(error) = job_status.error {
                    println!("   Error: {error:?}");
                }
                break;
            }
            DesignAutofillStatus::InProgress => {
                attempts += 1;
                if attempts >= max_attempts {
                    println!("⏰ Timeout waiting for job to complete");
                    break;
                }
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    println!("\n✨ Autofill example completed!");

    Ok(())
}
