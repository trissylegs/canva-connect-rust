use canva_connect::{
    auth::AccessToken,
    endpoints::exports::CreateDesignExportJobRequest,
    models::{ExportFormat, JobStatus},
    Client,
};
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

    let client = Client::new(AccessToken::new(access_token));

    println!("📤 Canva Connect Exports API Example");
    println!("===================================");

    // First, we need a design to export
    println!("\n1. Finding a design to export...");
    let designs = client.designs().list(None, None, None, None).await?;

    if designs.items.is_empty() {
        println!("❌ No designs found. Create a design first to test exports.");
        return Ok(());
    }

    let design = &designs.items[0];
    println!(
        "✅ Found design: {} (ID: {})",
        design.title.as_deref().unwrap_or("Untitled"),
        design.id
    );

    // Get available export formats
    println!("\n2. Getting available export formats...");
    let formats = client
        .exports()
        .get_design_export_formats(&design.id)
        .await?;

    // Build list of available formats
    let mut available_formats = Vec::new();
    if formats.formats.pdf.is_some() {
        available_formats.push((
            ExportFormat::Pdf {
                export_quality: None,
                size: None,
                pages: None,
            },
            "PDF",
        ));
    }
    if formats.formats.jpg.is_some() {
        available_formats.push((
            ExportFormat::Jpg {
                export_quality: None,
                quality: 80,
                height: None,
                width: None,
                pages: None,
            },
            "JPG",
        ));
    }
    if formats.formats.png.is_some() {
        available_formats.push((
            ExportFormat::Png {
                export_quality: None,
                height: None,
                width: None,
                pages: None,
            },
            "PNG",
        ));
    }
    if formats.formats.pptx.is_some() {
        available_formats.push((
            ExportFormat::Pptx {
                export_quality: None,
                pages: None,
            },
            "PPTX",
        ));
    }
    if formats.formats.gif.is_some() {
        available_formats.push((
            ExportFormat::Gif {
                export_quality: None,
                pages: None,
            },
            "GIF",
        ));
    }
    if formats.formats.mp4.is_some() {
        available_formats.push((
            ExportFormat::Mp4 {
                export_quality: None,
                pages: None,
            },
            "MP4",
        ));
    }

    if available_formats.is_empty() {
        println!("❌ No export formats available for this design");
        return Ok(());
    }

    println!(
        "✅ Available export formats: {}",
        available_formats
            .iter()
            .map(|(_, name)| *name)
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Use the first available format
    let (format_type, format_name) = &available_formats[0];
    println!("\n3. Exporting design as {format_name}...");

    // Create export job
    let export_job = client
        .exports()
        .create_design_export_job(&CreateDesignExportJobRequest {
            design_id: design.id.clone(),
            format: format_type.clone(),
        })
        .await?;

    println!("✅ Export job created: {}", export_job.job.id);
    println!("   - Status: {:?}", export_job.job.status);

    // Wait for export to complete
    println!("\n4. Waiting for export to complete...");
    let mut attempts = 0;
    let max_attempts = 30;

    loop {
        let job_status = client
            .exports()
            .get_design_export_job(&export_job.job.id)
            .await?;

        println!("   Export status: {:?}", job_status.job.status);

        match job_status.job.status {
            JobStatus::Success => {
                println!("✅ Export completed successfully!");
                if let Some(result) = &job_status.job.result {
                    println!("   Download URLs:");
                    for (i, url) in result.urls.iter().enumerate() {
                        println!("   {}. {}", i + 1, url.url);
                    }
                }
                break;
            }
            JobStatus::Failed => {
                println!("❌ Export job failed");
                if let Some(error) = &job_status.job.error {
                    println!("   Error: {error:?}");
                }
                break;
            }
            JobStatus::InProgress => {
                attempts += 1;
                if attempts >= max_attempts {
                    println!("⏰ Timeout waiting for export to complete");
                    break;
                }
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    // Try another format if available
    if available_formats.len() > 1 {
        let (second_format_type, second_format_name) = &available_formats[1];
        println!("\n5. Exporting in second format: {second_format_name}...");

        let second_export = client
            .exports()
            .create_design_export_job(&CreateDesignExportJobRequest {
                design_id: design.id.clone(),
                format: second_format_type.clone(),
            })
            .await?;

        println!("✅ Second export job created: {}", second_export.job.id);

        // Quick check of the second export status
        sleep(Duration::from_secs(1)).await;
        let second_status = client
            .exports()
            .get_design_export_job(&second_export.job.id)
            .await?;
        println!("   Second export status: {:?}", second_status.job.status);
    } else {
        println!("\n5. Only one export format available, skipping second export.");
    }

    println!("\n✨ Exports example completed!");
    println!("ℹ️  Export URLs are temporary and will expire after some time.");

    Ok(())
}
