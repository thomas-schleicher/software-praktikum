use common::api::config::Config;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Measurement {
    measurements: Vec<u64>,
}

#[derive(Debug, Error)]
enum ConfigurationPostError {
    #[error("API request failed: {0}")]
    API(#[from] reqwest::Error),

    #[error("Invalid response format: {0}")]
    Parse(#[from] serde_json::Error),
}

pub async fn send_config_to_ripe(client: &Client, config: Config) -> Result<Measurement, ConfigurationPostError> {
    let url = "https://atlas.ripe.net/api/v2/measurements";

    let res = client
        .post(url)
        .json(&config)
        .send()
        .await?
        .error_for_status()?;

    let measurement: Measurement = res.json().await?;
    Ok(measurement)
}