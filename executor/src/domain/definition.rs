use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Definition {
    #[serde(rename = "ping")]
    Ping(PingDefinition),

    #[serde(rename = "http")]
    Http(HttpDefinition),

    #[serde(rename = "traceroute")]
    Traceroute(TracerouteDefinition),
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
    pub port: u16,
    pub version: f32,
    pub more_extended_timing: bool,
    pub skip_dns_check: bool,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TracerouteDefinition {
    #[serde(rename = "type")]
    pub definition_type: String,
    pub af: u8,
    pub resolve_on_probe: bool,
    pub description: String,
    pub response_timeout: u32,
    pub protocol: String,
    pub packets: u32,
    pub size: u32,
    pub first_hop: u32,
    pub max_hops: u32,
    pub paris: u32,
    pub destination_option_size: u32,
    pub hop_by_hop_option_size: u32,
    pub dont_fragment: bool,
    pub skip_dns_check: bool,
    pub port: Option<u16>,
    pub target: String,
    pub tags: Option<Vec<String>>,
    pub interval: Option<u32>,
}

pub enum DefinitionTemplate {
    Ping(PingDefinition),
    Http(HttpDefinition),
    Traceroute(TracerouteDefinition),
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
            DefinitionTemplate::Traceroute(traceroute_definition) => {
                let mut clone = traceroute_definition.clone();
                clone.target = target.to_string();
                Definition::Traceroute(clone)
            }
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
impl HttpDefinition {
    pub fn template() -> Self {
        Self {
            definition_type: String::from("http"),
            af: 4,
            description: String::new(),
            resolve_on_probe: true,
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

    pub fn port(mut self, port: u16) -> Self {
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

#[allow(dead_code)]
impl TracerouteDefinition {
    pub fn template() -> Self {
        Self {
            definition_type: String::from("traceroute"),
            af: 4,
            resolve_on_probe: true,
            description: String::new(),
            response_timeout: 4000,
            protocol: String::from("UDP"),
            packets: 3,
            size: 48,
            first_hop: 1,
            max_hops: 32,
            paris: 16,
            destination_option_size: 0,
            hop_by_hop_option_size: 0,
            dont_fragment: false,
            skip_dns_check: false,
            port: None,
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

    pub fn response_timeout(mut self, response_timeout: u32) -> Self {
        self.response_timeout = response_timeout;
        self
    }

    pub fn protocol(mut self, protocol: String) -> Self {
        self.protocol = protocol;
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

    pub fn paris(mut self, paris: u32) -> Self {
        self.paris = paris;
        self
    }

    pub fn first_hop(mut self, first_hop: u32) -> Self {
        self.first_hop = first_hop;
        self
    }

    pub fn max_hops(mut self, max_hops: u32) -> Self {
        self.max_hops = max_hops;
        self
    }

    pub fn destination_option_size(mut self, destination_option_size: u32) -> Self {
        self.destination_option_size = destination_option_size;
        self
    }

    pub fn hop_by_hop_option_size(mut self, hop_by_hop_option_size: u32) -> Self {
        self.hop_by_hop_option_size = hop_by_hop_option_size;
        self
    }

    pub fn skip_dns_check(mut self, skip_dns_check: bool) -> Self {
        self.skip_dns_check = skip_dns_check;
        self
    }

    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = port;
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

    pub fn dont_fragment(mut self, dont_fragment: bool) -> Self {
        self.dont_fragment = dont_fragment;
        self
    }
}
