use reqwest;
use serde::{Deserialize, Serialize};
use crate::errors::AppError;
use axum::{response::IntoResponse, Json};

#[derive(Debug, Serialize, Deserialize)]
struct SensorLog {
    sensor_event: String,
    sensor_location: String
}

/// Get all trace logs in order
// pub async fn get_logs_handler() -> Result<Json<Vec<Value>>, AppError> {
pub async fn get_logs_handler() -> Result<impl IntoResponse, AppError> {
    // Ok(Json(alarm_state))
    let mut sensor_events: Vec<String> = vec![];

    // TODO example that works apparently:
    //curl -s 'http://10.99.29.154:16686/api/traces?service=orders&lookback=20m&prettyPrint=true&limit=1'
    // Define the URL for the Jaeger API endpoint to retrieve traces
    let url = "http://localhost:16686/api/traces?service=zigme";

    // Make a GET request to retrieve traces
    let response = reqwest::get(url).await?;

    let traces = response.json::<serde_json::Value>().await?;
    for trace in traces["data"].as_array().unwrap_or(&vec![]) {
        for span in trace["spans"].as_array().unwrap_or(&vec![]) {
            for logs in span["logs"].as_array().unwrap_or(&vec![]) {
                for field in logs["fields"].as_array().unwrap_or(&vec![]) {
                    if let Some(key) = field.get("key") {
                        if key == "sensor_event" {
                            if let Some(value) = field.get("value") {
                                sensor_events.push(value.as_str().unwrap_or_default().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // // Check if the request was successful
    // if response.status().is_success() {
    //     // Parse the JSON response body
    //     let traces = response.json::<serde_json::Value>().await?;

    //     // Iterate over each trace
    //     for trace in traces["data"].as_array().unwrap_or(&vec![]) {
    //         for span in trace["spans"].as_array().unwrap_or(&vec![]) {

    //         }
    //         // Extract the trace ID
    //         // let trace_id = trace["traceID"].as_str().unwrap_or_default();

    //         // Retrieve the trace details including logs
    //         // let trace_details_url = format!("http://localhost:16686/api/traces/{}", trace_id);
    //         // let trace_details_response = reqwest::get(&trace_details_url).await?;

    //         // // Check if the request for trace details was successful
    //         // if trace_details_response.status().is_success() {
    //         //     // Parse the JSON response body for trace details
    //         //     let trace_details = trace_details_response.json::<serde_json::Value>().await?;
    //         //     logs.push(trace_details);

    //             // Extract and process logs from the trace details
    //             // println!("{:?}", trace_details);
    //         // } else {
    //         //     println!("Failed to fetch trace details for trace ID {}", trace_id);
    //         // }
    //     }
    // } else {
    //     println!("Failed to fetch traces: {}", response.status());
    // }

    Ok(Json(sensor_events))
}
