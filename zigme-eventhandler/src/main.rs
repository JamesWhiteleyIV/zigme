use rumqttc::{Event, Packet, Publish};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::Serialize;
use std::time::Duration;
use std::env;
use anyhow::Result;
use tracing::{error, info, instrument};
use opentelemetry::{global, trace::{Tracer, TraceError}};
use opentelemetry_sdk::runtime::Tokio;

/// Payload passed along with alarm trigger with
/// information about what was triggered
#[derive(Debug, Serialize)]
pub struct AlarmTriggerPayload {
    title: String,
    message: String,
}

/// Send request to alarm_trigger endpoint of our API to trigger
/// whichever alarm(s) is/are currently set.
#[instrument]
async fn send_alarm_event_request(title: &str, message: &str) -> Result<()> {
    info!("zigme-eventhandler sending to alarm trigger endpoint: TITLE={:?} MESSAGE{:?}", title, message);
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

    info!("Response from alarm trigger endpoint: {:?}", &response);
    Ok(())
}

/// Handles parsing of incoming MQTT message and routing to our API
#[instrument]
async fn handle_incoming_mqtt_packet(p: Publish) -> Result<()> {
    if let Ok(payload) = serde_json::from_slice::<serde_json::Value>(&p.payload) {
        info!("zigme-eventhandler received packet TOPIC={:?} PAYLOAD={:?}", p.topic, payload);

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
 

#[instrument]
#[tokio::main]
async fn main() {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline().install_batch(Tokio).unwrap();
    // tracer.in_span("doing_work", |cx| {
    //     // Traced app logic here...
    // });

    dotenv::dotenv().ok();
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

    global::shutdown_tracer_provider(); // export remaining spans
}

