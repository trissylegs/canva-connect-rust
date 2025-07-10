//! Observability support for the Canva Connect client.
//!
//! This module provides OpenTelemetry integration for tracing API requests
//! and monitoring client performance. All HTTP requests are automatically
//! instrumented with spans that include request metadata, response status,
//! and Canva API request IDs for correlation. Enable observability features by
//! adding the `observability` feature flag.
//!
//! ## Setup
//!
//! ```toml
//! [dependencies]
//! canva-connect = { version = "0.1", features = ["observability"] }
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! # #[cfg(feature = "observability")]
//! # async fn example() -> Result<(), String> {
//! use canva_connect::{Client, auth::AccessToken, observability::init_tracing};
//!
//! // Initialize tracing with OTLP exporter
//! let _guard = init_tracing("canva-connect-app", "http://localhost:4317").await?;
//!
//! let client = Client::new(AccessToken::new("token"));
//!
//! // API calls will now be traced automatically
//! let user = client.user().get_me().await.unwrap();
//! println!("User ID: {}", user.user_id);
//!
//! Ok(())
//! # }
//! ```

#[cfg(feature = "observability")]
pub use self::implementation::*;

#[cfg(feature = "observability")]
mod implementation {
    use opentelemetry_otlp::WithExportConfig;
    use tracing_opentelemetry::OpenTelemetryLayer;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    /// Initialize distributed tracing with OpenTelemetry.
    ///
    /// This sets up a tracing subscriber that exports traces to an OTLP endpoint
    /// (like Jaeger or OTEL Collector).
    ///
    /// # Arguments
    ///
    /// * `service_name` - The name of your service for trace identification
    /// * `otlp_endpoint` - The OTLP gRPC endpoint URL (e.g., "http://localhost:4317")
    ///
    /// # Returns
    ///
    /// Returns a guard that should be kept alive for the duration of the application.
    /// When dropped, it will flush any remaining traces.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use canva_connect::observability::init_tracing;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let _guard = init_tracing("my-app", "http://localhost:4317").await?;
    ///     
    ///     // Your application code here
    ///     // All canva-connect API calls will be traced
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn init_tracing(
        service_name: &str,
        otlp_endpoint: &str,
    ) -> Result<TracingGuard, String> {
        // Configure OTLP exporter
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(otlp_endpoint),
            )
            .with_trace_config(opentelemetry_sdk::trace::config().with_resource(
                opentelemetry_sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", service_name.to_string()),
                    opentelemetry::KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                ]),
            ))
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .map_err(|e| format!("Failed to install tracer: {e}"))?;

        // Create tracing layer
        let telemetry_layer = OpenTelemetryLayer::new(tracer);

        // Set up the subscriber with both console and OpenTelemetry layers
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(false)
                    .with_level(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .with(telemetry_layer)
            .try_init()
            .map_err(|e| format!("Failed to initialize tracing subscriber: {e}"))?;

        Ok(TracingGuard)
    }

    /// Guard that ensures proper cleanup of tracing resources.
    ///
    /// Keep this alive for the duration of your application to ensure
    /// traces are properly flushed when the application exits.
    pub struct TracingGuard;

    impl Drop for TracingGuard {
        fn drop(&mut self) {
            // Shutdown the global tracer provider to flush remaining spans
            opentelemetry::global::shutdown_tracer_provider();
        }
    }

    // Note: reqwest-tracing middleware is complex to configure with newer versions
    // We'll rely on manual instrumentation in the client for now
}

#[cfg(not(feature = "observability"))]
mod no_op {
    //! No-op implementations when observability feature is disabled.

    /// No-op tracing guard when observability is disabled.
    pub struct TracingGuard;

    /// Initialize tracing (no-op when observability feature is disabled).
    pub async fn init_tracing(
        _service_name: &str,
        _otlp_endpoint: &str,
    ) -> Result<TracingGuard, String> {
        eprintln!("Warning: Observability feature not enabled. Tracing will not be active.");
        Ok(TracingGuard)
    }
}

#[cfg(not(feature = "observability"))]
pub use no_op::*;
