use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::configuration::{http::HttpConfig, ping::PingConfig, probes::Probes, topology::Topology};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    interval: Option<u32>,
    mode: Option<String>,
    #[serde(rename = "ping")]
    ping_configuration: Option<PingConfig>,
    #[serde(rename = "http")]
    http_configuration: Option<HttpConfig>,
    probes: Option<Probes>, //TODO: think about if I dont want to have this as optional
    topology: Option<Topology>,
}

#[derive(Default, Debug)]
pub struct ConfigBuilder {
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    interval: Option<u32>,
    mode: Option<String>,
    ping_configuration: Option<PingConfig>,
    http_configuration: Option<HttpConfig>,
    probes: Option<Probes>,
    topology: Option<Topology>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_time(mut self, val: impl Into<DateTime<Utc>>) -> Self {
        self.start_time = Some(val.into());
        self
    }

    pub fn end_time(mut self, val: impl Into<DateTime<Utc>>) -> Self {
        self.end_time = Some(val.into());
        self
    }

    pub fn interval(mut self, val: impl Into<u32>) -> Self {
        self.interval = Some(val.into());
        self
    }

    pub fn mode(mut self, val: impl Into<String>) -> Self {
        self.mode = Some(val.into());
        self
    }

    pub fn ping_configuration(mut self, val: impl Into<PingConfig>) -> Self {
        self.ping_configuration = Some(val.into());
        self
    }

    pub fn http_configuration(mut self, val: impl Into<HttpConfig>) -> Self {
        self.http_configuration = Some(val.into());
        self
    }

    pub fn probes(mut self, val: impl Into<Probes>) -> Self {
        self.probes = Some(val.into());
        self
    }

    pub fn topology(mut self, val: impl Into<Topology>) -> Self {
        self.topology = Some(val.into());
        self
    }

    pub fn build(self) -> Result<Configuration, &'static str> {
        Ok(Configuration {
            start_time: self.start_time,
            end_time: self.end_time,
            interval: self.interval,
            mode: self.mode,
            ping_configuration: self.ping_configuration,
            http_configuration: self.http_configuration,
            probes: self.probes,
            topology: self.topology,
        })
    }
}