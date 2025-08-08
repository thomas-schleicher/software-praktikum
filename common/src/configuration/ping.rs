use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PingConfig {
    packet_count: u32,
    size: u32,
}

#[derive(Default, Debug)]
pub struct PingConfigBuilder {
    packet_count: u32,
    size: u32,
}

impl PingConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn packet_count(mut self, val: impl Into<u32>) -> Self {
        self.packet_count = val.into();
        self
    }

    pub fn size(mut self, val: impl Into<u32>) -> Self {
        self.size = val.into();
        self
    }

    pub fn build(self) -> Result<PingConfig, &'static str> {
        Ok(PingConfig {
            packet_count: self.packet_count,
            size: self.size,
        })
    }
}