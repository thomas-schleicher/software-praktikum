use crate::domain::config::Config;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Measurement {
    measurements: Vec<u64>,
}

#[derive(Debug, Error)]
pub enum RipeMeasurementCreationError {
    #[error("Failed to reach RIPE Atlas API: {0}")]
    Network(#[source] reqwest::Error),

    #[error("Ripe Atlas API returned an error: {status} - {body}")]
    API { status: StatusCode, body: String },

    #[error("Failed to parse expected JSON response body: {0}")]
    ResponseFormat(#[from] serde_json::Error),
}

pub async fn create_ripe_measurement(
    client: &Client,
    config: Config,
    api_key: &str,
) -> Result<Measurement, RipeMeasurementCreationError> {
    let url = "https://atlas.ripe.net/api/v2/measurements";

    let res = client
        .post(url)
        .header("Authorization", format!("Key {}", api_key))
        .json(&config)
        .send()
        .await
        .map_err(RipeMeasurementCreationError::Network)?;

    let status = res.status();
    let text = res
        .text()
        .await
        .map_err(RipeMeasurementCreationError::Network)?;

    if !status.is_success() {
        return Err(RipeMeasurementCreationError::API { status, body: text });
    }

    let measurement: Measurement =
        serde_json::from_str(&text).map_err(RipeMeasurementCreationError::ResponseFormat)?;
    Ok(measurement)
}
