use crate::db::RedisClient;
use crate::errors::AppError;
use axum::{
    extract::State,
    response::IntoResponse,
    Json, 
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use reqwest;
use std::env;
use super::alarm_state;

const PUSHOVER_URI: &str = "https://api.pushover.net/1/messages.json";

/// Payload passed along with alarm trigger with
/// information about what was triggered
#[derive(Debug, Deserialize)]
pub struct AlarmTriggerPayload {
    title: String,
    message: String,
}


/// Response containing what was triggered 
#[derive(Debug, Serialize)]
pub struct AlarmTriggerResponse{
    triggers: Vec<String>
}


/// Struct containing payload body to Pushover API
#[derive(Debug, Serialize)]
struct PushoverPayload {
    token: String,
    user: String,
    title: String,
    message: String,
    priority: u8,
    sound: Option<String>
}


/// Send request to local server to start alarm siren
async fn send_local_siren_start() -> Result<String, AppError> {
    Ok("TODO".to_string())
}

/// Send request to local server to stop alarm siren
async fn send_local_siren_stop() -> Result<String, AppError> {
    Ok("TODO".to_string())
}

/// Send request to pushover to trigger notification on phone
async fn send_phone_notifications(title: &str, message: &str) -> Result<String, AppError> {
    let payload = PushoverPayload {
        token: env::var("ZIGME_PUSHOVER_API_TOKEN")?,
        user: env::var("ZIGME_PUSHOVER_GROUP_KEY")?,
        title: title.to_string(),
        message: message.to_string(),
        priority: 0,
        sound: None
    };
    let client = reqwest::Client::new();
    let response = client
        .post(PUSHOVER_URI)
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}


/// Send request to pushover to trigger alarm on phone
async fn send_phone_alarms(title: &str, message: &str) -> Result<String, AppError> {
    let payload = PushoverPayload {
        token: env::var("ZIGME_PUSHOVER_API_TOKEN")?,
        user: env::var("ZIGME_PUSHOVER_GROUP_KEY")?,
        title: title.to_string(),
        message: message.to_string(),
        priority: 1,
        sound: Some("persistent".to_string())
    };
    let client = reqwest::Client::new();
    let response = client
        .post(PUSHOVER_URI)
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}


/// Submit a trigger which will trigger any set alarms/notifications
/// from the redis db
pub async fn post_alarm_trigger_handler(
    State(redis_client): State<Arc<RedisClient>>,
    Json(payload): Json<AlarmTriggerPayload>,
) -> Result<impl IntoResponse, AppError> {
    let local_siren: Option<bool> = redis_client.get(alarm_state::STATE_LOCAL_SIREN).await?;
    let phone_alarms: Option<bool> = redis_client.get(alarm_state::STATE_PHONE_ALARMS).await?;
    let phone_notifications: Option<bool> = redis_client.get(alarm_state::STATE_PHONE_NOTIFICATIONS).await?;

    let mut results: Vec<String> = vec![];
    if let Some(local_siren) = local_siren {
        if local_siren {
            send_local_siren_start().await?;
            results.push(alarm_state::STATE_LOCAL_SIREN.to_string());
        }
    }

    if let Some(phone_alarms) = phone_alarms {
        if phone_alarms {
            send_phone_alarms(&payload.title, &payload.message).await?;
            results.push(alarm_state::STATE_PHONE_ALARMS.to_string());
        }
    }

    if let Some(phone_notifications) = phone_notifications {
        if phone_notifications {
            send_phone_notifications(&payload.title, &payload.message).await?;
            results.push(alarm_state::STATE_PHONE_NOTIFICATIONS.to_string());
        }
    }

    Ok(Json(results))
}


