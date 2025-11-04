use common::configuration::configuration::Configuration;
use futures::future::try_join_all;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct ProbeInformation {
    pub probe_id: u32,
    pub address_v4: String,
    // pub country_code: String,
    pub is_anchor: bool,
    pub fqdn: Option<String>,
}

impl<'de> Deserialize<'de> for ProbeInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let probe_id = value
            .get("probe")
            .or_else(|| value.get("id"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| serde::de::Error::missing_field("id or probe"))?
            as u32;

        let address_v4 = value
            .get("ip_v4")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .into();

        // let country_code = value
        //     .get("country")
        //     .and_then(|v| v.as_str())
        //     .unwrap_or_default()
        //     .into();

        let is_anchor = value
            .get("is_anchor")
            .and_then(|v| v.as_bool())
            .or_else(|| {
                value
                    .get("type")
                    .and_then(|v| v.as_str())
                    .map(|s| s.eq_ignore_ascii_case("anchor"))
            })
            .unwrap_or(false);

        let fqdn = value.get("fqdn").and_then(|v| v.as_str()).map(String::from);

        Ok(Self {
            probe_id,
            address_v4,
            // country_code,
            is_anchor,
            fqdn,
        })
    }
}

#[derive(Debug, Error)]
pub enum FetchProbeInformationError {
    #[error("Failed to reach RIPE Atlas API: {0}")]
    Network(#[source] reqwest::Error),

    #[error("RIPE Atlas API returned an error: {status} - {body}")]
    API { status: StatusCode, body: String },

    #[error("Failed to parse expected JSON response body: {0}")]
    ResponseFormat(#[from] serde_json::Error),

    #[error("Configuration Error: {0}")]
    ConfigurationError(String),
}

pub async fn fetch_information(
    client: &Client,
    config: &Configuration,
) -> Result<Vec<ProbeInformation>, FetchProbeInformationError> {
    match (&config.anchors, &config.probes) {
        (Some(_), Some(_)) => Err(FetchProbeInformationError::ConfigurationError(
            "Configuration contains both anchors and probes.".to_string(),
        )),
        (None, None) => Err(FetchProbeInformationError::ConfigurationError(
            "No anchors or probes are defined in the provided configuration.".to_string(),
        )),
        (Some(anchor_config), None) => {
            let futures = anchor_config
                .anchors
                .iter()
                .map(|anchor_id| fetch_single(client, *anchor_id, "anchors"));

            let anchors = try_join_all(futures).await?;
            Ok(anchors)
        }
        (None, Some(probe_config)) => {
            let futures = probe_config
                .probes
                .iter()
                .map(|probe_id| fetch_single(client, *probe_id, "probes"));

            let probes = try_join_all(futures).await?;
            Ok(probes)
        }
    }
}

async fn fetch_single(
    client: &Client,
    id: u32,
    endpoint: &str,
) -> Result<ProbeInformation, FetchProbeInformationError> {
    let url = format!("https://atlas.ripe.net/api/v2/{endpoint}/{id}");
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|err| FetchProbeInformationError::Network(err))?;

    let status = res.status();
    let body = res
        .text()
        .await
        .map_err(|err| FetchProbeInformationError::Network(err))?;

    if !status.is_success() {
        return Err(FetchProbeInformationError::API { status, body });
    }

    let probe_information: ProbeInformation =
        serde_json::from_str(&body).map_err(FetchProbeInformationError::ResponseFormat)?;

    Ok(probe_information)
}
