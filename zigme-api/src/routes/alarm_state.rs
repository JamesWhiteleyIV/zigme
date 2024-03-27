use super::{STATE_LOCAL_SIREN, STATE_PHONE_ALARMS, STATE_PHONE_NOTIFICATIONS};
use crate::db::RedisClient;
use crate::errors::AppError;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Struct containing state of various alarms and notifications
#[derive(Debug, Deserialize, Serialize)]
pub struct AlarmState {
    phone_alarms: Option<bool>,
    phone_notifications: Option<bool>,
    local_siren: Option<bool>,
}

async fn get_alarm_state(redis_client: Arc<RedisClient>) -> Result<AlarmState, AppError> {
    let phone_alarms: Option<bool> = redis_client.get(STATE_PHONE_ALARMS)?;
    let phone_notifications: Option<bool> = redis_client.get(STATE_PHONE_NOTIFICATIONS)?;
    let local_siren: Option<bool> = redis_client.get(STATE_LOCAL_SIREN)?;

    let alarm_state = AlarmState {
        phone_alarms,
        phone_notifications,
        local_siren,
    };

    Ok(alarm_state)
}

/// Get current alarm states
pub async fn get_alarm_state_handler(
    State(redis_client): State<Arc<RedisClient>>,
) -> Result<Json<AlarmState>, AppError> {
    let alarm_state = get_alarm_state(redis_client).await?;
    Ok(Json(alarm_state))
}

/// Update 1 or more alarm states and return all states as response
pub async fn put_alarm_state_handler(
    State(redis_client): State<Arc<RedisClient>>,
    Json(payload): Json<AlarmState>,
) -> Result<Json<AlarmState>, AppError> {
    if let Some(phone_alarms) = payload.phone_alarms {
        redis_client.set(STATE_PHONE_ALARMS, phone_alarms)?;
    }
    if let Some(phone_notifications) = payload.phone_notifications {
        redis_client.set(STATE_PHONE_NOTIFICATIONS, phone_notifications)?;
    }
    if let Some(local_siren) = payload.local_siren {
        redis_client.set(STATE_LOCAL_SIREN, local_siren)?;
    }

    let alarm_state = get_alarm_state(redis_client).await?;
    Ok(Json(alarm_state))
}

