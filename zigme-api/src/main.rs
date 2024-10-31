#![warn(clippy::pedantic)]
mod db;
mod errors;
mod routes;

use axum::{
    routing::{put, get, post},
    Router,
};
use std::{collections::HashMap, env, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let redis_uri: String = env::var("ZIGME_REDIS_URI").unwrap_or("redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(db::RedisClient::new(&redis_uri));

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route(
            "/alarm_state",
            get(routes::alarm_state::get_alarm_state_handler)
                .put(routes::alarm_state::put_alarm_state_handler),
        )
        .route(
            "/alarm_trigger",
            post(routes::alarm_trigger::post_alarm_trigger_handler),
        )
        // TODO
        // .route("/device_state_change", get(routes::device_state_change::get_device_states_handler).put(routes::device_state_change::put_device_state_change_handler))
        .route("/events", get(routes::events::get_events_handler))
        .with_state(redis_client);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3020"))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
