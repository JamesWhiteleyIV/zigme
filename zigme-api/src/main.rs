#![warn(clippy::pedantic)]
mod db;
mod errors;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use std::{env, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis_uri: String = env::var("ZIGME_REDIS_URI").unwrap_or("redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(db::RedisClient::new(&redis_uri));
    tracing::debug!("using redis uri: {}", redis_uri);

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
        .route("/events", get(routes::events::get_events_handler))
        .with_state(redis_client);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3020"))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
