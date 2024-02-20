use super::{AlarmEvent, REDIS_EVENTS_LIST_KEY};
use crate::db::RedisClient;
use crate::errors::AppError;
use axum::{extract::State, Json};
use std::sync::Arc;
use tracing::instrument;

/// Get most recent events list
#[instrument(skip(redis_client))]
pub async fn get_events_handler(
    State(redis_client): State<Arc<RedisClient>>,
) -> Result<Json<Vec<AlarmEvent>>, AppError> {
    let events: Vec<String> = redis_client.get_list(REDIS_EVENTS_LIST_KEY)?;
    let mut event_json: Vec<AlarmEvent> = vec![];

    for event in events {
        let event: AlarmEvent = serde_json::from_str(&event)?;
        event_json.push(event);
    }
    // Reverse the order so we get latest first
    event_json.reverse();
    Ok(Json(event_json))
}
