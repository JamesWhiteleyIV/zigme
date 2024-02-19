// TODO: possibly roll back - cannot figure out why tracing isn't all going up!!!
#![warn(clippy::pedantic)]
use anyhow::Result;
use opentelemetry::global;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use rumqttc::{Event, Packet, Publish};
use serde_json::json;
use std::env;
use std::time::Duration;
use tracing::{error, info, instrument, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

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

    tracing_subscriber::registry()
        // log to open telemetry
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        // log to stdout
        .with(fmt::Layer::default())
        // set log level to INFO
        .with(tracing_subscriber::filter::Targets::new().with_default(Level::INFO))
        .try_init()
        .unwrap();
}

#[instrument]
async fn handle_incoming_event(event: Event) -> Result<()> {
    if let Event::Incoming(Packet::Publish(p)) = event {
        handle_incoming_mqtt_packet(p).await?; 
    }
    Ok(())
}

/// Attempt to parse zigbee2mqtt/front-door => front-door
fn parse_topic(topic: &str) -> &str {
    if let Some(second_item) = topic.split('/').nth(1) {
        second_item
    } else {
        topic
    }
}

/// Handles parsing of incoming MQTT message and routing to our API
#[instrument]
async fn handle_incoming_mqtt_packet(p: Publish) -> Result<()> {
    let topic = p.topic;
    let sensor_location = parse_topic(&topic);
    let payload = serde_json::from_slice::<serde_json::Value>(&p.payload)?;
    // info!(payload = payload.to_string());

    // Handle vibration sensor trigger
    if let Some(vibration) = payload.get("vibration") {
        if vibration == true {
            match send_alarm_event_request(sensor_location, "VIBRATION").await {
                Ok(()) => info!(sensor_event = format!("{} VIBRATION", sensor_location)),
                Err(e) => error!("{}", e)
            }
        }
    }

    // Handle contact sensor trigger
    if let Some(contact) = payload.get("contact") {
        if contact == false {
            match send_alarm_event_request(sensor_location, "OPENED").await {
                Ok(()) => info!(sensor_event = format!("{} OPENED", sensor_location)),
                Err(e) => error!("{}", e)
            }
        } else {
            info!(sensor_event = format!("{} CLOSED", sensor_location));
        }
    }

    Ok(())
}

/// Send request to alarm trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
#[instrument]
async fn send_alarm_event_request(topic: &str, message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    client
        .post(env::var("ZIGME_API_ALARM_TRIGGER_URI").unwrap())
        .json(&json!({
            "title": topic.to_string(),
            "message": message.to_string()
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

#[instrument]
#[tokio::main]
async fn main() {
    // Setup telemetry tracer
    setup_tracer();

    // Load in environment vars
    dotenv::dotenv().ok();

    let mqtt_host: String = env::var("ZIGME_MQTT_HOST").unwrap();
    let mqtt_port: u16 = env::var("ZIGME_MQTT_PORT")
        .unwrap()
        .parse()
        .expect("Could not parse ZIGME_MQTT_PORT as u16");
    let mqtt_topic: String = env::var("ZIGME_MQTT_TOPIC").unwrap();
    let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe(mqtt_topic, QoS::ExactlyOnce)
        .await
        .unwrap();

    loop {
        if let Ok(event) = eventloop.poll().await {
            match handle_incoming_event(event).await {
                Ok(()) => {},
                Err(e) => error!("{}", e)
            }
        }
    }
}
