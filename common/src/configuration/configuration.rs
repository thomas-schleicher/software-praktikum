use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::configuration::{
    anchors::Anchors, http::HttpConfig, ping::PingConfig, probes::Probes, topology::Topology,
    traceroute::TracerouteConfig,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub interval: Option<u32>,
    pub mode: Option<String>,
    #[serde(rename = "ping")]
    pub ping_configuration: Option<PingConfig>,
    #[serde(rename = "http")]
    pub http_configuration: Option<HttpConfig>,
    #[serde(rename = "traceroute")]
    pub traceroute_configuration: Option<TracerouteConfig>,
    pub probes: Option<Probes>,
    pub anchors: Option<Anchors>,
    pub topology: Option<Topology>,
}

#[derive(Default, Debug)]
pub struct ConfigBuilder {
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    interval: Option<u32>,
    mode: Option<String>,
    ping_configuration: Option<PingConfig>,
    http_configuration: Option<HttpConfig>,
    traceroute_configuration: Option<TracerouteConfig>,
    probes: Option<Probes>,
    anchors: Option<Anchors>,
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

    pub fn traceroute_configuration(mut self, val: impl Into<TracerouteConfig>) -> Self {
        self.traceroute_configuration = Some(val.into());
        self
    }

    pub fn probes(mut self, val: impl Into<Probes>) -> Self {
        self.probes = Some(val.into());
        self
    }

    pub fn anchors(mut self, val: impl Into<Anchors>) -> Self {
        self.anchors = Some(val.into());
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
            traceroute_configuration: self.traceroute_configuration,
            probes: self.probes,
            anchors: self.anchors,
            topology: self.topology,
        })
    }
}
