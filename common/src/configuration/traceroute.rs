use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TracerouteConfig {}

#[derive(Debug, Default)]
pub struct TracerouteConfigBuilder {}

impl TracerouteConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TracerouteConfig {
        TracerouteConfig {}
    }
}
