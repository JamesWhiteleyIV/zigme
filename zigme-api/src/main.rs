mod db;
mod errors;
mod routes;
use routes::alarm_state;
use routes::alarm_trigger;

use axum::{
    routing::{get, post},
    Router,
};
use db::RedisClient;
use std::sync::Arc;
use dotenv;

const REDIS_URI: &str = "redis://127.0.0.1/";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let redis_client = Arc::new(RedisClient::new(REDIS_URI).await);

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route(
            "/alarm_state",
            get(alarm_state::get_alarm_state_handler).put(alarm_state::put_alarm_state_handler)
        )
        .route(
            "/alarm_trigger",
            post(alarm_trigger::post_alarm_trigger_handler)
        )
        .with_state(redis_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
