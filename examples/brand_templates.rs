use canva_connect::{
    auth::AccessToken, endpoints::brand_templates::ListBrandTemplatesRequest, Client,
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

    let client = Client::new(AccessToken::new(access_token));

    println!("🎨 Canva Connect Brand Templates API Example");
    println!("===========================================");

    // List brand templates
    println!("\n1. Listing brand templates...");
    let templates = client
        .brand_templates()
        .list(&ListBrandTemplatesRequest::default())
        .await?;

    println!("✅ Found {} brand templates", templates.items.len());

    if templates.items.is_empty() {
        println!(
            "ℹ️  No brand templates found. Create some brand templates in Canva to see them here."
        );
        return Ok(());
    }

    // Display template summaries
    for (i, template) in templates.items.iter().enumerate() {
        println!("   {}. {} (ID: {})", i + 1, template.title, template.id);
    }

    // Get details for the first template
    let first_template = &templates.items[0];
    println!(
        "\n2. Getting details for template: {}",
        first_template.title
    );

    let template_details = client.brand_templates().get(&first_template.id).await?;

    println!("✅ Template details retrieved:");
    println!("   - ID: {}", template_details.brand_template.id);
    println!("   - Title: {}", template_details.brand_template.title);
    println!(
        "   - View URL: {}",
        template_details.brand_template.view_url
    );
    println!(
        "   - Create URL: {}",
        template_details.brand_template.create_url
    );
    println!(
        "   - Created: {}",
        template_details.brand_template.created_at
    );
    println!(
        "   - Updated: {}",
        template_details.brand_template.updated_at
    );

    if let Some(thumbnail) = &template_details.brand_template.thumbnail {
        println!("   - Thumbnail: {}", thumbnail.url);
    }

    // Get dataset for the template
    println!("\n3. Getting dataset for template...");
    match client
        .brand_templates()
        .get_dataset(&first_template.id)
        .await
    {
        Ok(dataset) => {
            println!("✅ Dataset retrieved:");
            println!("   - Field count: {}", dataset.dataset.len());

            for (i, (field_name, field)) in dataset.dataset.iter().enumerate() {
                println!("   {}. Field Name: {}", i + 1, field_name);

                let (field_type, label, description, required) = match field {
                    canva_connect::models::DataField::Text {
                        label,
                        description,
                        required,
                    } => ("Text", label, description, required),
                    canva_connect::models::DataField::Image {
                        label,
                        description,
                        required,
                    } => ("Image", label, description, required),
                    canva_connect::models::DataField::Chart {
                        label,
                        description,
                        required,
                    } => ("Chart", label, description, required),
                };

                println!("      Type: {field_type}");
                if let Some(label) = label {
                    println!("      Label: {label}");
                }
                if let Some(description) = description {
                    println!("      Description: {description}");
                }
                if let Some(required) = required {
                    println!("      Required: {required}");
                }
            }
        }
        Err(e) => {
            println!("ℹ️  Could not retrieve dataset: {e}");
            println!("   This is normal if the template doesn't have a dataset configured.");
        }
    }

    // If we have multiple templates, show a few more
    if templates.items.len() > 1 {
        println!(
            "\n4. Showing details for {} additional templates...",
            std::cmp::min(2, templates.items.len() - 1)
        );

        for template in templates.items.iter().skip(1).take(2) {
            println!("\n   Template: {}", template.title);
            match client.brand_templates().get(&template.id).await {
                Ok(details) => {
                    println!("   ✅ ID: {}", details.brand_template.id);
                    println!("   🔗 View URL: {}", details.brand_template.view_url);
                    if let Some(thumbnail) = &details.brand_template.thumbnail {
                        println!("   📸 Thumbnail: {}", thumbnail.url);
                    }
                }
                Err(e) => {
                    println!("   ❌ Error getting details: {e}");
                }
            }
        }
    }

    println!("\n✨ Brand templates example completed!");

    Ok(())
}
