use common::configuration::configuration::Configuration;
use futures::future::try_join_all;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct ProbeInformation {
    #[serde(skip)]
    pub probe_id: u32,
    pub address_v4: String,
    // pub country_code: String,
    pub is_anchor: bool,
    pub status: ProbeStatus,
}

#[derive(Debug, Deserialize)]
pub struct ProbeStatus {
    pub id: u32,
}

enum ProbeStatusCode {
    Connected = 1,
}

#[derive(Debug, Error)]
pub enum FetchProbeInformationError {
    #[error("Failed to reach RIPE Atlas API: {0}")]
    Network(#[source] reqwest::Error),

    #[error("RIPE Atlas API returned an error: {status} - {body}")]
    API { status: StatusCode, body: String },

    #[error("Probe {probe_id} is offline")]
    Offline { probe_id: u32 },

    #[error("Failed to parse expected JSON response body: {0}")]
    ResponseFormat(#[from] serde_json::Error),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),
}

pub async fn fetch_all_probes(
    client: &Client,
    config: &Configuration,
) -> Result<Vec<ProbeInformation>, FetchProbeInformationError> {
    let Some(probe_config) = &config.probes else {
        return Err(FetchProbeInformationError::ConfigurationError(
            "No probes are not defined in the provided configuration.".to_string(),
        ));
    };

    if probe_config.probes.is_empty() {
        return Err(FetchProbeInformationError::ConfigurationError(
            "Probe list in configuration is empty.".to_string(),
        ));
    }

    let futures = probe_config
        .probes
        .iter()
        .map(|probe_id| fetch_probe_information(client, probe_id));

    let probes = try_join_all(futures).await?;
    Ok(probes)
}

async fn fetch_probe_information(
    client: &Client,
    probe_id: &str,
) -> Result<ProbeInformation, FetchProbeInformationError> {
    let url = format!("https://atlas.ripe.net/api/v2/probes/{}/", probe_id);

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|err| FetchProbeInformationError::Network(err))?; //maybe handle this differently

    let status = res.status();
    let text = res
        .text()
        .await
        .map_err(FetchProbeInformationError::Network)?;

    if !status.is_success() {
        return Err(FetchProbeInformationError::API { status, body: text });
    }

    let mut probe_information: ProbeInformation =
        serde_json::from_str(&text).map_err(FetchProbeInformationError::ResponseFormat)?;
    probe_information.probe_id = probe_id.parse().map_err(|_| {
        FetchProbeInformationError::ConfigurationError(format!(
            "ProbeID {} could not be parsed correctly.",
            probe_id
        ))
    })?;

    if probe_information.status.id != ProbeStatusCode::Connected as u32 {
        return Err(FetchProbeInformationError::Offline {
            probe_id: probe_information.probe_id,
        });
    }

    Ok(probe_information)
}
