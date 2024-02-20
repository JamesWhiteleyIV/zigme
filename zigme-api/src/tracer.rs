use opentelemetry::global;
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Create and setup our global tracer for use with #instrument
pub fn setup_telemetry() {
    // Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // Sets up the machinery needed to export data to Jaeger
    // There are other OTel crates that provide pipelines for the vendors
    // mentioned earlier.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("zigme")
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let layer = tracing_subscriber::filter::Targets::new().with_default(Level::INFO);
    tracing_subscriber::registry()
        // Set log level to info
        .with(layer)
        // log to open telemetry
        .with(opentelemetry)
        // log to stdout
        .with(fmt::Layer::default())
        .try_init()
        .unwrap();
}
