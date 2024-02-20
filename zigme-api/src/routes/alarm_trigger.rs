use super::{
    AlarmEvent, REDIS_EVENTS_LIST_KEY, REDIS_EVENTS_LIST_MAX_ITEMS, STATE_LOCAL_SIREN,
    STATE_PHONE_ALARMS, STATE_PHONE_NOTIFICATIONS,
};
use crate::{db::RedisClient, errors::AppError};
use axum::{extract::State, response::IntoResponse, Json};
use reqwest;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use tracing::instrument;

/// Struct containing payload body to Pushover API
#[derive(Debug, Serialize)]
struct PushoverPayload {
    token: String,
    user: String,
    title: String,
    message: String,
    priority: u8,
    sound: Option<String>,
}

/// Send request to local server to start alarm siren
// async fn send_local_siren_start() -> Result<String, AppError> {
//     Ok("TODO".to_string())
// }

/// Send request to local server to stop alarm siren
// async fn send_local_siren_stop() -> Result<String, AppError> {
//     Ok("TODO".to_string())
// }

/// Send request to pushover to trigger notification on phone
#[instrument]
async fn send_phone_notifications(title: &str, message: &str) -> Result<String, AppError> {
    let payload = PushoverPayload {
        token: env::var("ZIGME_PUSHOVER_API_TOKEN")?,
        user: env::var("ZIGME_PUSHOVER_GROUP_KEY")?,
        title: title.to_string(),
        message: message.to_string(),
        priority: 0,
        sound: None,
    };
    let client = reqwest::Client::new();
    let response = client
        .post(env::var("ZIGME_PUSHOVER_URI")?)
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

/// Send request to pushover to trigger alarm on phone
#[instrument]
async fn send_phone_alarms(title: &str, message: &str) -> Result<String, AppError> {
    let payload = PushoverPayload {
        token: env::var("ZIGME_PUSHOVER_API_TOKEN")?,
        user: env::var("ZIGME_PUSHOVER_GROUP_KEY")?,
        title: title.to_string(),
        message: message.to_string(),
        priority: 1,
        sound: Some("persistent".to_string()),
    };
    let client = reqwest::Client::new();
    let response = client
        .post(env::var("ZIGME_PUSHOVER_URI")?)
        .json(&payload)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

/// Submit a trigger which will trigger any set alarms/notifications
/// from the redis db
#[instrument(skip(redis_client))]
pub async fn post_alarm_trigger_handler(
    State(redis_client): State<Arc<RedisClient>>,
    Json(payload): Json<AlarmEvent>,
) -> Result<impl IntoResponse, AppError> {
    let local_siren: Option<bool> = redis_client.get(STATE_LOCAL_SIREN)?;
    let phone_alarms: Option<bool> = redis_client.get(STATE_PHONE_ALARMS)?;
    let phone_notifications: Option<bool> = redis_client.get(STATE_PHONE_NOTIFICATIONS)?;

    let mut results: Vec<String> = vec![];
    if let Some(local_siren) = local_siren {
        if local_siren {
            // TODO
            //send_local_siren_start().await?;
            results.push(STATE_LOCAL_SIREN.to_string());
        }
    }

    if let Some(phone_alarms) = phone_alarms {
        if phone_alarms {
            send_phone_alarms(&payload.title, &payload.message).await?;
            results.push(STATE_PHONE_ALARMS.to_string());
        }
    }

    if let Some(phone_notifications) = phone_notifications {
        if phone_notifications {
            send_phone_notifications(&payload.title, &payload.message).await?;
            results.push(STATE_PHONE_NOTIFICATIONS.to_string());
        }
    }

    // Add new event to list, removing oldest event if necessary
    redis_client.append_list(REDIS_EVENTS_LIST_KEY, serde_json::to_string(&payload)?)?;
    redis_client.remove_oldest_item(REDIS_EVENTS_LIST_KEY, REDIS_EVENTS_LIST_MAX_ITEMS)?;

    Ok(Json(results))
}
