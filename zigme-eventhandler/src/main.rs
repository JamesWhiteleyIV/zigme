#![warn(clippy::pedantic)]
use anyhow::Result;
use opentelemetry::{global, trace::{TraceContextExt, Tracer}, Context};
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Request};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use std::{collections::HashMap, env};
use std::time::Duration;
use tracing::{error, info, instrument, Level, Span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::{
      propagation::Injector,
  };
  
// Main event loop for subscribing to mqtt topic
#[tokio::main]
async fn main() {
    // Setup telemetry tracer
    setup_tracer();
    dotenv::dotenv();
    match send_alarm_event_request("SENSOR_LOCATION", "MESSAGE").await {
        Ok(()) => info!("OK"),
        Err(e) => error!("{e}")
    }
}

// Create and setup our global tracer for use with #instrument
fn setup_tracer() {
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
        // log to open telemetry
        .with(opentelemetry)
        // log to stdout
        .with(fmt::Layer::default())
        // set log level to INFO
        .with(layer)
        .try_init()
        .unwrap();
}

// pub fn send_trace<T>(mut request: Request) -> Result<Request> {
//     global::get_text_map_propagator(|propagator| {
//         let context = Span::current().context();
//         propagator.inject_context(&context, &mut MetadataInjector(request.metadata_mut()))
//     });

//     Ok(request)
// } 

/// Serializable datastructure to hold the opentelemetry propagation context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataInjector(HashMap<String, String>);

impl MetadataInjector {
    fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn inject(context: &opentelemetry::Context) -> Self {
        global::get_text_map_propagator(|propagator| {
            let mut propagation_context = MetadataInjector::empty();
            propagator.inject_context(context, &mut propagation_context);
            propagation_context
        })
    }
}

impl Injector for MetadataInjector {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
}
}


/// Send request to alarm trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
#[instrument]
async fn send_alarm_event_request(sensor_location: &str, message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    // let span = global::tracer("zigme").start("say hello");
    // let cx = Context::current_with_span(span);
    // dbg!(cx);

    // Retrieve the current span
    let span = Span::current();
    // Retrieve the current context
    let cx = span.context();
    
    // Inject context into 
    let injector = MetadataInjector::inject(&cx);
    dbg!(&injector);

    let (key, val) = injector.0.get_key_value("uber-trace-id").unwrap();

    // let header_map = HeaderMap::from(injector);
    let response = client
        .post(env::var("ZIGME_API_ALARM_TRIGGER_URI")?)
        .header(key, val)
        .json(&json!({
            "title": sensor_location.to_string(),
            "message": message.to_string()
        })).send().await?;

    // span.in_scope(|| {       
    //     let propagation_context = PropagationContext::inject(&span.context());
    //     let spanned_message = SpannedMessage::new(propagation_context, message);
    //     // kafka_send_message(producer, queue, spanned_message)
    //     let response = req.send().await?;
    //     })

    // let response = req.send().await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err().into())
    }
}
