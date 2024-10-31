#![warn(clippy::pedantic)]
use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, Publish, QoS};
use serde_json::json;
use std::env;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, util::SubscriberInitExt};
use std::fs;
use std::path::Path;
use tokio::time::sleep;

// Main event loop for subscribing to mqtt topic
#[tokio::main]
async fn main() {
    // Spawn a task to update the health check file
    let health_file_path = "/tmp/zigme_eventhandler_health_check.txt";
    tokio::spawn(async move {
        loop {
            fs::write(health_file_path, "Healthy").expect("Unable to write file");
            sleep(Duration::from_secs(30)).await; // Update every 30 seconds
        }
    });

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let mqtt_host: String = env::var("MQTT_HOST").unwrap_or("localhost".to_string());
    let mqtt_port: u16 = env::var("MQTT_PORT")
        .unwrap_or("1883".to_string())
        .parse()
        .unwrap();
    let mqtt_topic: String = env::var("MQTT_TOPIC").unwrap_or("zigbee2mqtt/+".to_string());

    tracing::info!("listening on mqtt address: {}:{}", mqtt_host, mqtt_port);
    tracing::info!("subscribing to mqtt topic: {}", mqtt_topic);

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
async fn handle_incoming_publish_packet(publish: Publish) {
    let topic = publish.topic;
    let payload = publish.payload;

    if let Err(e) = route_payload(&topic, &payload).await {
        tracing::error!("{}", e);
    }
}

async fn route_payload(topic: &str, payload: &[u8]) -> Result<()> {
    let sensor_location = parse_topic(topic);
    // Convert bytes object to serde_json::Value
    let payload = serde_json::from_slice::<serde_json::Value>(payload)?;

    // {"battery":100,"contact":true,"device_temperature":26,"linkquality":102,"power_outage_count":81,"voltage":3015}
    // {"battery":100,"device_temperature":21,"linkquality":72,"power_outage_count":9,"strength":5,"vibration":false,"voltage":3115}
    // {"angle":68,"angle_x":-4,"angle_x_absolute":94,"angle_y":72,"angle_y_absolute":18,"angle_z":18,"battery":100,"device_temperature":26,"linkquality":138,"power_outage_count":12,"strength":2,"vibration":false,"voltage":3055,"x_axis":-78,"y_axis":1068,"z_axis":344}
    // sensorName::: battery: int, linkquality: int, device_temperature: int, contact: Bool OR vibration: Bool

    // Handle contact sensor trigger
    if let Some(contact) = payload.get("contact") {
        // We only care if the sensor has lost contact (e.g. window/door has been opened)
        if contact == false {
            send_alarm_event_request(sensor_location, "OPENED").await?;
        }
    }

    // Handle vibration sensor trigger
    if let Some(vibration) = payload.get("vibration") {
        // We only care if the sensor has sensed a vibration
        if vibration == true {
            send_alarm_event_request(sensor_location, "VIBRATION").await?;
        }
    }

    // TODO
    // send_device_state_change_request(sensor_location, payload).await?;

    Ok(())
}

// Get the current datetime for Los Angeles
fn get_timestamp() -> String {
    let la_datetime: DateTime<Tz> = Utc::now().with_timezone(&Tz::America__Los_Angeles);
    la_datetime.format("%Y-%m-%d %l:%M:%S%P").to_string()
}

/// Send request to alarm trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
async fn send_alarm_event_request(sensor_location: &str, message: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let host = env::var("ZIGME_API_HOST").unwrap_or("localhost".to_string());
    let port = env::var("ZIGME_API_PORT").unwrap_or("3020".to_string());
    let uri = format!("http://{}:{}/alarm_trigger", host, port);

    let response = client
        .post(uri)
        .json(&json!({
            "title": sensor_location.to_string(),
            "message": message.to_string(),
            "timestamp": get_timestamp()
        }))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err().into())
    }
}

/// Send request to update the state of this device
async fn send_device_state_change_request(sensor_location: &str, state: serde_json::Value) -> Result<()> {
    let client = reqwest::Client::new();
    let host = env::var("ZIGME_API_HOST").unwrap_or("localhost".to_string());
    let port = env::var("ZIGME_API_PORT").unwrap_or("3020".to_string());
    let uri = format!("http://{}:{}/device_state_change", host, port);

    let response = client
        .put(uri)
        .json(&json!({
            "timestamp": get_timestamp(),
            "sensor_location": sensor_location.to_string(),
            "state": state,
        }))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(response.error_for_status().unwrap_err().into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timestamp() {
        dbg!(get_timestamp());
    }

    #[tokio::test]
    async fn test_send_alarm_event_request() {
        dbg!(send_alarm_event_request("sensor_location1", "message").await.unwrap());
    }

}