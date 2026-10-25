use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::api::results::AggregatedMeasurement;

const SECONDS_PER_DAY: u64 = 86_400;

#[derive(Debug, Error)]
pub enum FetchMeasurementDataError {
    #[error("Failed to reach RIPE Atlas API: {0}")]
    Network(#[source] reqwest::Error),

    #[error("RIPE Atlas API returned an error: {status} - {body}")]
    API { status: StatusCode, body: String },

    #[error("Failed to parse expected JSON response body: {0}")]
    ResponseFormat(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
struct MeasurementMetadata {
    start_time: u64,
    stop_time: Option<u64>,
}

pub async fn get_measurement_data(
    client: &Client,
    measurement_id: &str,
) -> Result<Vec<AggregatedMeasurement>, FetchMeasurementDataError> {
    let metadata: MeasurementMetadata = fetch_json(
        client,
        &format!(
            "https://atlas.ripe.net/api/v2/measurements/{}/",
            measurement_id
        ),
    )
    .await?;

    let end_time = metadata.stop_time.unwrap_or_else(current_unix_time);
    let mut measurements = Vec::new();
    let mut window_start = metadata.start_time;

    // The results endpoint rejects requests spanning 7 days or more, so the
    // measurement's lifetime is queried one day at a time and concatenated.
    while window_start < end_time {
        let window_end = (window_start + SECONDS_PER_DAY - 1).min(end_time);
        let day: Vec<AggregatedMeasurement> = fetch_json(
            client,
            &format!(
                "https://atlas.ripe.net/api/v2/measurements/{}/results/?start={}&stop={}",
                measurement_id, window_start, window_end
            ),
        )
        .await?;
        measurements.extend(day);
        window_start = window_end + 1;
    }

    Ok(measurements)
}

async fn fetch_json<T: DeserializeOwned>(
    client: &Client,
    url: &str,
) -> Result<T, FetchMeasurementDataError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(FetchMeasurementDataError::Network)?;

    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(FetchMeasurementDataError::Network)?;

    if !status.is_success() {
        return Err(FetchMeasurementDataError::API { status, body: text });
    }

    Ok(serde_json::from_str(&text)?)
}

fn current_unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the unix epoch")
        .as_secs()
}
