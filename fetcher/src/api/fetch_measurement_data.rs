use reqwest::{Client, StatusCode};
use thiserror::Error;

use crate::api::results::AggregatedMeasurement;

#[derive(Debug, Error)]
pub enum FetchMeasurementDataError {
    #[error("Failed to reach RIPE Atlas API: {0}")]
    Network(#[source] reqwest::Error),

    #[error("RIPE Atlas API returned an error: {status} - {body}")]
    API { status: StatusCode, body: String },

    #[error("Failed to parse expected JSON response body: {0}")]
    ResponseFormat(#[from] serde_json::Error),
}

pub async fn get_measurement_data(
    client: &Client,
    measurement_id: &str,
) -> Result<Vec<AggregatedMeasurement>, FetchMeasurementDataError> {
    let url = format!(
        "https://atlas.ripe.net/api/v2/measurements/{}/results/",
        measurement_id
    );

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| FetchMeasurementDataError::Network(err))?;

    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(|err| FetchMeasurementDataError::Network(err))?;

    if !status.is_success() {
        return Err(FetchMeasurementDataError::API { status, body: text });
    }

    let measurement_data = serde_json::from_str(&text)
        .map_err(|err| FetchMeasurementDataError::ResponseFormat(err))?;

    Ok(measurement_data)
}
