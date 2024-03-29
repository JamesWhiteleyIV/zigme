#![warn(clippy::pedantic)]
mod tracer;

use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use serde_json::json;
use std::env;
use std::time::Duration;
use tracing::{error, instrument};

// Main event loop for subscribing to mqtt topic
#[tokio::main]
async fn main() {
    // Setup telemetry tracer
    tracer::setup_telemetry();

    let mqtt_host: String = env::var("ZIGME_MQTT_HOST").unwrap_or("localhost".to_string());
    let mqtt_port: u16 = env::var("ZIGME_MQTT_PORT")
        .unwrap_or("1883".to_string())
        .parse()
        .expect("Could not parse ZIGME_MQTT_PORT as u16");
    let mqtt_topic: String = env::var("ZIGME_MQTT_TOPIC").unwrap_or("zigbee2mqtt/+".to_string());
    let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .subscribe(mqtt_topic, QoS::ExactlyOnce)
        .await
        .unwrap();

    loop {
        // We only care about incoming publish packets
        if let Ok(Event::Incoming(Packet::Publish(publish))) = eventloop.poll().await {
            handle_incoming_publish_packet(publish).await;
        }
    }
}

/// Attempt to parse zigbee2mqtt/front-door => front-door
fn parse_topic(topic: &str) -> &str {
    if let Some(second_item) = topic.split('/').nth(1) {
        second_item
    } else {
        topic
    }
}

// Main handler for parsing mqtt event and forwarding any messages
// we care about to our API to trigger alarm.
#[instrument]
async fn handle_incoming_publish_packet(publish: Publish) {
    let topic = publish.topic;
    let payload = publish.payload;

    if let Err(e) = route_payload(&topic, &payload).await {
        error!("{}", e);
    }
}

#[instrument]
async fn route_payload(topic: &str, payload: &[u8]) -> Result<()> {
    let sensor_location = parse_topic(topic);

    // Convert bytes object to serde_json::Value
    let payload = serde_json::from_slice::<serde_json::Value>(payload)?;

    // Handle vibration sensor trigger
    if let Some(vibration) = payload.get("vibration") {
        // We only care if the sensor has sensed a vibration
        if vibration == true {
            send_alarm_event_request(sensor_location, "VIBRATION").await?;
        }
    }

    // Handle contact sensor trigger
    if let Some(contact) = payload.get("contact") {
        // We only care if the sensor has lost contact (e.g. window/door has been opened)
        if contact == false {
            send_alarm_event_request(sensor_location, "OPENED").await?;
        }
    }

    Ok(())
}

// Get the current datetime for Los Angeles
fn get_la_timestamp() -> String {
    let la_datetime: DateTime<Tz> = Utc::now().with_timezone(&Tz::America__Los_Angeles);
    la_datetime.format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

/// Send request to alarm trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
#[instrument]
async fn send_alarm_event_request(sensor_location: &str, message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let response = client
        .post(env::var("ZIGME_API_ALARM_TRIGGER_URI").unwrap())
        .json(&json!({
            "title": sensor_location.to_string(),
            "message": message.to_string(),
            "timestamp": get_la_timestamp()
        }))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err().into())
    }
}
