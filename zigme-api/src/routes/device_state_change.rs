use crate::{db::RedisClient, errors::AppError};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceStateChange {
    timestamp: String,
    sensor_location: String,
    state: DeviceState
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeviceState {
    linkquality: Option<i32>,
    battery: Option<i32>,
    device_temperature: Option<i32>,
    contact: Option<bool>,
    vibration: Option<bool>,
}

/// Update state on a device, e.g. contact
pub async fn put_device_state_change_handler(State(redis_client): State<Arc<RedisClient>>, 
Json(payload): Json<DeviceStateChange>) -> Result<(), AppError> {
    let key = format!("device_state:{}", &payload.sensor_location);
    redis_client.set(&key, payload);
    Ok(())
}

/// Get all device states
pub async fn get_device_states_handler(State(redis_client): State<Arc<RedisClient>>) -> Result<Json<Vec<DeviceStateChange>>, AppError> {
    let keys = redis_client.keys("device_state:*")?;
    let mut device_state_changes: Vec<DeviceStateChange> = vec![];
    for key in keys {
        let device_state_change = redis_client.get(&key)?.unwrap();
        tracing::info!("{:#?}", device_state_change);
        device_state_changes.push(device_state_change);
    }
    Ok(Json(device_state_changes))
}

