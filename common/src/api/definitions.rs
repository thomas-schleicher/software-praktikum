use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "type")]
    def_type: String,
    af: u8,
    resolve_on_probe: bool, 
    description: String, 
    packets: u32,
    size: u32,
    skip_dns_check: bool,
    include_probe_id: bool,
    target: Ipv4Addr, // as string
    tags: Vec<String>,
    interval: u32,
}