use crate::db::RedisClient;
use crate::errors::AppError;
use axum::{
    extract::State,
    Json, 
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, instrument, Level, info_span, Span};

pub const STATE_PHONE_ALARMS: &str = "phone_alarms";
pub const STATE_PHONE_NOTIFICATIONS: &str = "phone_notifications";
pub const STATE_LOCAL_SIREN: &str = "local_siren";

/// Struct containing state of various alarms and notifications
#[derive(Debug, Deserialize, Serialize)]
pub struct AlarmState {
    phone_alarms: Option<bool>,
    phone_notifications: Option<bool>,
    local_siren: Option<bool>,
}

async fn get_alarm_state(redis_client: Arc<RedisClient>) -> Result<AlarmState, AppError> {
    let phone_alarms: Option<bool> = redis_client.get(STATE_PHONE_ALARMS).await?;
    let phone_notifications: Option<bool> = redis_client.get(STATE_PHONE_NOTIFICATIONS).await?;
    let local_siren: Option<bool> = redis_client.get(STATE_LOCAL_SIREN).await?;

    Ok(AlarmState{
        phone_alarms,
        phone_notifications,
        local_siren,
    })
}

/// Update 1 or more alarm states and return all states as response
pub async fn put_alarm_state_handler(
    State(redis_client): State<Arc<RedisClient>>,
    Json(payload): Json<AlarmState>,
) -> Result<Json<AlarmState>, AppError> {
    if let Some(phone_alarms) = payload.phone_alarms {
        let _: () = redis_client.set(STATE_PHONE_ALARMS, phone_alarms).await?;
    }
    if let Some(phone_notifications) = payload.phone_notifications {
        let _: () = redis_client
            .set(STATE_PHONE_NOTIFICATIONS, phone_notifications)
            .await?;
    }
    if let Some(local_siren) = payload.local_siren {
        let _: () = redis_client.set(STATE_LOCAL_SIREN, local_siren).await?;
    }

    let alarm_state = get_alarm_state(redis_client).await?;
    Ok(Json(alarm_state))
}

/// Get current alarm states
pub async fn get_alarm_state_handler(
    State(redis_client): State<Arc<RedisClient>>,
) -> Result<Json<AlarmState>, AppError> {
    let alarm_state = get_alarm_state(redis_client).await?;
    Ok(Json(alarm_state))
}
