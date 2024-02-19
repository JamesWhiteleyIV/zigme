use rumqttc::{Event, Packet, Publish};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Serialize;
use std::time::Duration;
use std::env;
use anyhow::Result;
use tracing::{error, info, instrument, Level};
use opentelemetry::global;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Payload passed along with alarm trigger with
/// information about what was triggered
#[derive(Debug, Serialize)]
pub struct AlarmTriggerPayload {
    title: String,
    message: String,
}

/// Send request to alarm_trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
// #[instrument]
async fn send_alarm_event_request(title: &str, message: &str) -> Result<()> {
    info!("sending TITLE={:?} MESSAGE{:?}", title, message);
    let endpoint = env::var("ZIGME_API_ALARM_TRIGGER_URI")?;
    let payload = AlarmTriggerPayload {
        title: title.to_string(),
        message: message.to_string(),
    };
    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;

    info!("response: {:?}", &response);
    Ok(())
}

/// Handles parsing of incoming MQTT message and routing to our API
// #[instrument]
async fn handle_incoming_mqtt_packet(p: Publish) -> Result<()> {
    if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&p.payload) {
        info!("received mqtt packet TOPIC={:?} PAYLOAD={:?}", p.topic, payload);

        // handle vibration sensor trigger
        if let Some(vibration) = payload.get("vibration") {
            if vibration == true {
                send_alarm_event_request(p.topic.as_str(), "Vibration Triggered").await?;
            }
        }

        // handle contact sensor trigger
        if let Some(contact) = payload.get("contact") {
            if contact == false {
                send_alarm_event_request(p.topic.as_str(), "Sensor Opened").await?;
            }
        }
    }

    Ok(())
}
 

// #[instrument]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // Sets up the machinery needed to export data to Jaeger
    // There are other OTel crates that provide pipelines for the vendors
    // mentioned earlier.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("zigme-eventhandler")
        .install_batch(opentelemetry::runtime::Tokio).unwrap();
        // .install_simple().unwrap();
    // Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // FmtSubscriber
    // TODO does this do anything?
    // let subscriber = tracing_subscriber::fmt()
    //     // Use a more compact, abbreviated log format
    //     .compact()
    //     // Display source code file paths
    //     .with_file(true)
    //     // Display source code line numbers
    //     .with_line_number(true)
    //     // Display the thread ID an event was recorded on
    //     .with_thread_ids(true)
    //     // Don't display the event's target (module path)
    //     .with_target(false)
    //     // Set log level
    //     .with_max_level(Level::INFO)
    //     // Build the subscriber
    //     .finish();

    // Continue logging to stdout
    // let layer = fmt::Layer::default();
    // The SubscriberExt and SubscriberInitExt traits are needed to extend the
    // Registry to accept `opentelemetry (the OpenTelemetryLayer type).
    let filter = tracing_subscriber::filter::Targets::new()
    .with_default(Level::INFO);
    // .with_target("tower_http::trace::on_response", Level::TRACE)
    // .with_target("tower_http::trace::on_request", Level::TRACE)

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .with(filter)
        .try_init().unwrap();

    // TODO: what does this do????
    // tracer.in_span("doing_work", |cx| {
    //     // Traced app logic here...
    //});

    let mqtt_host = env::var("ZIGME_MQTT_HOST").unwrap();
    let mqtt_port: u16 = env::var("ZIGME_MQTT_PORT").unwrap().parse().expect("Could not parse ZIGME_MQTT_PORT as u16");
    let mqtt_topic = env::var("ZIGME_MQTT_TOPIC").unwrap();

    let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe(mqtt_topic, QoS::ExactlyOnce).await.unwrap();

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {

                match handle_incoming_mqtt_packet(p).await {
                    Ok(_) => {},
                    Err(e) => error!("{}", e)
                }
            },
            // throw away anything other than incoming packet publishes 
            Ok(_) => {},
            Err(_) => {}
        }
    }

    // TODO: is this required to push all remaining traces??
    // global::shutdown_tracer_provider(); // export remaining spans
}

