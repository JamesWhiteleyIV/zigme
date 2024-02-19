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
use std::env;
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let port = env::var("ZIGME_API_PORT").unwrap();
    let redis_uri = env::var("ZIGME_REDIS_URI").unwrap();
    let redis_client = Arc::new(RedisClient::new(&redis_uri).await);

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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
