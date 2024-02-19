mod db;
mod errors;
mod routes;
use routes::alarm_state;
use routes::alarm_trigger;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::{get, post},
    Router,
};
use db::RedisClient;
use std::sync::Arc;
use std::env;
use std::time::Duration;
use dotenv;
use tracing::{error, instrument, Level, info_span, Span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::global;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};

#[tokio::main]
async fn main() {
    setup_tracer();

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
                        "http_request",
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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// Create and setup our global tracer for use with #instrument
fn setup_tracer() {
    // Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // Sets up the machinery needed to export data to Jaeger
    // There are other OTel crates that provide pipelines for the vendors
    // mentioned earlier.
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("zigme")
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let layer = tracing_subscriber::filter::Targets::new().with_default(Level::INFO);
    tracing_subscriber::registry()
        // log to open telemetry
        .with(opentelemetry)
        // log to stdout
        .with(fmt::Layer::default())
        // set log level to INFO
        .with(layer)
        .try_init()
        .unwrap();
}