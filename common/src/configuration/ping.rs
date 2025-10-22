use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PingConfig {
    pub packet_count: u32,
    pub size: u32,
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

    pub fn build(self) -> PingConfig {
        PingConfig {
            packet_count: self.packet_count,
            size: self.size,
        }
    }
}
