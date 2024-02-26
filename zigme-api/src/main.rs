#![warn(clippy::pedantic)]
mod db;
mod errors;
mod routes;
mod tracer;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, post},
    Router,
};
use std::{env, sync::Arc, time::Duration};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

#[tokio::main]
async fn main() {
    tracer::setup_telemetry();

    let redis_url: String = env::var("ZIGME_REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(db::RedisClient::new(&redis_url));

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
        .with_state(redis_client)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "api_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:3020"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
