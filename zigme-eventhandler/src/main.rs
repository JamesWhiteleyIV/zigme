use rumqttc::{Event, Packet, SubscribeFilter};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use std::env;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let mqtt_host = env::var("ZIGME_MQTT_HOST").unwrap();
    let mqtt_port: u16 = env::var("ZIGME_MQTT_PORT").unwrap().parse().expect("Could not parse ZIGME_MQTT_PORT as u16");
    let mqtt_topics = env::var("ZIGME_MQTT_TOPICS").unwrap();
    let mqtt_topics: Vec<&str> = mqtt_topics.split(',').collect();
    let mut topics: Vec<SubscribeFilter> = vec![];
    for topic in mqtt_topics {
        topics.push(SubscribeFilter::new(topic.to_string(), QoS::AtMostOnce))
    }

    let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe_many(topics).await.unwrap();

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let payload: serde_json::Value = serde_json::from_slice(&p.payload).expect("Failed to deserialize");
                let topic = p.topic;
                println!("Incoming = {:?}, {:?}", topic, payload);
            },
            _ => {}

        }
    }
}

// some notes...
// Ok(Event::Incoming(Packet::PingResp)) |
// Ok(Event::Outgoing(Outgoing::PingReq)) => {},
// Ok(Event::Incoming(i)) => {
//     println!("Incoming = {:?}", i);
// },
// Ok(Event::Outgoing(o)) => {
//     println!("Outgoing = {:?}", o);
// },
// Err(e) => {
//     {}
//     // println!("Error = {:?}", e);
//     // XXX: Here I have to re-subscribe to the topics :(
//     // client.subscribe("#", QoS::AtMostOnce).await.unwrap();
// }