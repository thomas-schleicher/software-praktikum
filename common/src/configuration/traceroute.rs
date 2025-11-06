use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TracerouteConfig {
    pub protocol: String,
    pub packets: u32,
    pub size: u32,
    pub first_hop: u32,
    pub max_hops: u32,
    pub paris: u32,
    // pub destination_option_size: u32,
    // pub hob_by_hob_option_size: u32,
    pub dont_fragment: bool,
    pub port: Option<u16>,
}

#[derive(Debug, Default)]
pub struct TracerouteConfigBuilder {
    pub protocol: String,
    pub packets: u32,
    pub size: u32,
    pub first_hop: u32,
    pub max_hops: u32,
    pub paris: u32,
    // pub destination_option_size: u32,
    // pub hob_by_hob_option_size: u32,
    pub dont_fragment: bool,
    pub port: Option<u16>,
}

impl TracerouteConfigBuilder {
    pub fn new() -> Self {
        Self::default()
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

    pub fn first_hop(mut self, first_hop: u32) -> Self {
        self.first_hop = first_hop;
        self
    }

    pub fn max_hops(mut self, max_hops: u32) -> Self {
        self.max_hops = max_hops;
        self
    }

    pub fn paris(mut self, paris: u32) -> Self {
        self.paris = paris;
        self
    }

    pub fn dont_fragment(mut self, dont_fragment: bool) -> Self {
        self.dont_fragment = dont_fragment;
        self
    }

    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = port;
        self
    }

    pub fn build(self) -> TracerouteConfig {
        TracerouteConfig {
            protocol: self.protocol,
            packets: self.packets,
            size: self.size,
            first_hop: self.first_hop,
            max_hops: self.max_hops,
            paris: self.paris,
            dont_fragment: self.dont_fragment,
            port: self.port,
        }
    }
}
