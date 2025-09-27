use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Probe {
    #[serde(skip)]
    pub probe_id: u32,
    pub address_v4: String,
    pub country_code: String,
    pub is_anchor: bool,
    pub status: ProbeStatus
}

#[derive(Debug, Deserialize)]
pub struct ProbeStatus {
    pub id: u32,
}

#[derive(Debug, Error)]
pub enum ProbeError {
    #[error("API request failed: {0}")]
    API(#[from] reqwest::Error),

    #[error("Probe {0} is offline")]
    Offline(String),

    #[error("Invalid response format: {0}")]
    Parse(#[from] serde_json::Error),
}

pub async fn fetch_probe_information(client: &Client, probe_id: &str) -> Result<Probe, ProbeError> {
    let url = format!("https://atlas.ripe.net/api/v2/probes/{}/", probe_id);
    
    let res = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let mut probe: Probe = res.json().await?;
    probe.probe_id = probe_id.parse().unwrap();    //This shouldnt fail because the api request was successful
    let probe = probe;                      // Make it imutable again

    if probe.status.id != 1 {
        return Err(ProbeError::Offline(probe_id.to_string()));
    }

    Ok(probe)
}