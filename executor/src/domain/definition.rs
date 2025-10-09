use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Definition {
    #[serde(rename = "ping")]
    Ping(PingDefinition),

    #[serde(rename = "http")]
    Http(HttpDefinition),
}

#[derive(Debug, Clone, Serialize)]
pub struct PingDefinition {
    #[serde(rename = "type")]
    pub definition_type: String,
    pub af: u8,
    pub description: String,
    pub resolve_on_probe: bool,
    pub packets: u32,
    pub size: u32,
    pub skip_dns_check: bool,
    pub include_probe_id: bool,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpDefinition {
    #[serde(rename = "type")]
    pub definition_type: String,
    pub af: u8,
    pub description: String,
    pub resolve_on_probe: bool,
    pub path: String,
    pub header_bytes: u32,
    pub method: String,
    pub extended_timing: bool,
    pub port: u32,
    pub version: f32,
    pub more_extended_timing: bool,
    pub skip_dns_check: bool,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
}

pub enum DefinitionTemplate {
    Ping(PingDefinition),
    Http(HttpDefinition),
}

impl DefinitionTemplate {
    pub fn with_target(&self, target: &str) -> Definition {
        match self {
            DefinitionTemplate::Ping(ping_definition) => {
                let mut clone = ping_definition.clone();
                clone.target = target.to_string();
                Definition::Ping(clone)
            }
            DefinitionTemplate::Http(http_definition) => {
                let mut clone = http_definition.clone();
                clone.target = target.to_string();
                Definition::Http(clone)
            }
        }
    }
}

impl PingDefinition {
    pub fn template() -> Self {
        Self {
            definition_type: String::from("ping"),
            af: 4,
            description: String::new(),
            resolve_on_probe: true,
            packets: 3,
            size: 48,
            skip_dns_check: false,
            include_probe_id: false,
            target: String::new(),
            tags: None,
            interval: None,
        }
    }

    pub fn af(mut self, af: u8) -> Self {
        self.af = af;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn resolve_on_probe(mut self, resolve: bool) -> Self {
        self.resolve_on_probe = resolve;
        self
    }

    pub fn packets(mut self, packets: u32) -> Self {
        self.packets = packets;
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn skip_dns_check(mut self, skip_dns_check: bool) -> Self {
        self.skip_dns_check = skip_dns_check;
        self
    }

    pub fn include_probe_id(mut self, include_probe_id: bool) -> Self {
        self.include_probe_id = include_probe_id;
        self
    }

    pub fn tags(mut self, tags: Option<Vec<String>>) -> Self {
        self.tags = tags;
        self
    }

    pub fn interval(mut self, interval: Option<u32>) -> Self {
        self.interval = interval;
        self
    }
}

impl HttpDefinition {
    pub fn template() -> Self {
        Self {
            definition_type: String::from("http"),
            af: 4,
            description: String::new(),
            resolve_on_probe: false,
            path: String::new(),
            header_bytes: 0,
            method: String::from("GET"),
            extended_timing: false,
            port: 80,
            version: 1.1,
            more_extended_timing: false,
            skip_dns_check: false,
            target: String::new(),
            tags: None,
            interval: None,
        }
    }

    pub fn af(mut self, af: u8) -> Self {
        self.af = af;
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn resolve_on_probe(mut self, resolve: bool) -> Self {
        self.resolve_on_probe = resolve;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn header_bytes(mut self, bytes: u32) -> Self {
        self.header_bytes = bytes;
        self
    }

    pub fn method(mut self, method: String) -> Self {
        self.method = method;
        self
    }

    pub fn extended_timing(mut self, extended_timing: bool) -> Self {
        self.extended_timing = extended_timing;
        self
    }

    pub fn port(mut self, port: u32) -> Self {
        self.port = port;
        self
    }

    pub fn version(mut self, version: f32) -> Self {
        self.version = version;
        self
    }

    pub fn more_extended_timing(mut self, more_extended_timing: bool) -> Self {
        self.more_extended_timing = more_extended_timing;
        self
    }

    pub fn skip_dns_check(mut self, skip_dns_check: bool) -> Self {
        self.skip_dns_check = skip_dns_check;
        self
    }

    pub fn target(mut self, target: String) -> Self {
        self.target = target.into();
        self
    }

    pub fn tags(mut self, tags: Option<Vec<String>>) -> Self {
        self.tags = tags;
        self
    }

    pub fn interval(mut self, interval: Option<u32>) -> Self {
        self.interval = interval;
        self
    }
}
