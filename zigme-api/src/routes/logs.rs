use reqwest;
use crate::errors::AppError;
use serde_json::Value;
use axum::Json;

/// Get all trace logs in order
// pub async fn get_logs_handler() -> Result<Json<Vec<Value>>, AppError> {
pub async fn get_logs_handler() -> Result<Json<Value>, AppError> {
    // Ok(Json(alarm_state))
    let mut logs: Vec<Value> = vec![];

    // TODO example that works apparently:
    //curl -s 'http://10.99.29.154:16686/api/traces?service=orders&lookback=20m&prettyPrint=true&limit=1'
    // Define the URL for the Jaeger API endpoint to retrieve traces
    let url = "http://localhost:16686/api/traces?service=zigme";

    // https://jaeger-query:16686/api/traces/{trace-id-hex-string}

    // Make a GET request to retrieve traces
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response body
        let traces = response.json::<serde_json::Value>().await?;

        // Iterate over each trace
        for trace in traces["data"].as_array().unwrap_or(&vec![]) {
            // Extract the trace ID
            let trace_id = trace["traceID"].as_str().unwrap_or_default();
            logs.push(trace_details);

            // Retrieve the trace details including logs
            // let trace_details_url = format!("http://localhost:16686/api/traces/{}", trace_id);
            // let trace_details_response = reqwest::get(&trace_details_url).await?;

            // // Check if the request for trace details was successful
            // if trace_details_response.status().is_success() {
            //     // Parse the JSON response body for trace details
            //     let trace_details = trace_details_response.json::<serde_json::Value>().await?;
            //     logs.push(trace_details);

                // Extract and process logs from the trace details
                // println!("{:?}", trace_details);
            // } else {
            //     println!("Failed to fetch trace details for trace ID {}", trace_id);
            // }
        }
    } else {
        println!("Failed to fetch traces: {}", response.status());
    }

    Ok(Json(Value::Array(logs)))
}
