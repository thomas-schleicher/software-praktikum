use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpConfig {
    path: String,
    header_bytes: String,
    port: u16,
    version: f32,
    method: String,
}

#[derive(Debug, Default)]
pub struct HttpConfigBuilder {
    path: String,
    header_bytes: String,
    port: u16,
    version: f32,
    method: String,
}

impl HttpConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn path(mut self, val: impl Into<String>) -> Self {
        self.path = val.into();
        self
    }

    pub fn header_bytes(mut self, val: impl Into<String>) -> Self {
        self.header_bytes = val.into();
        self
    }

    pub fn port(mut self, val: impl Into<u16>) -> Self {
        self.port = val.into();
        self
    }

    pub fn version(mut self, val: impl Into<f32>) -> Self {
        self.version = val.into();
        self
    }

    pub fn method(mut self, val: impl Into<String>) -> Self {
        self.method = val.into();
        self
    }

    pub fn build(self) -> Result<HttpConfig, &'static str> {
        Ok(HttpConfig {
            path: self.path,
            header_bytes: self.header_bytes,
            port: self.port,
            version: self.version,
            method: self.method,
        })
    }
}