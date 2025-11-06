use common::configuration::traceroute::TracerouteConfig;
use dialoguer::{Input, Select};
use std::error::Error;

enum Protocol {
    UDP,
    ICMP,
    TCP,
}

impl Protocol {
    fn kind(&self) -> &'static str {
        match self {
            Self::ICMP => "ICMP",
            Self::UDP => "UDP",
            Self::TCP => "TCP",
        }
    }
}

pub fn prompt_traceroute_config() -> TracerouteConfig {
    TracerouteConfig {
        protocol: Protocol::UDP,
        packets: 3,
        size: 64,
        first_hop: 1,
        max_hops: 30,
        paris: 0,
        dont_fragment: false,
        port: 33434,
    }
}

fn prompt_protocol() -> Result<String, Box<dyn Error>> {
    let options = vec!["ICMP", "UDP", "TCP"];
    let selection = Select::new()
        .with_prompt("Choose protocol")
        .items(&options)
        .default(1)
        .interact()
        .map_err(|e| format!("Failed to make protocol selection: {e}"))?;
    Ok(options[selection].to_string())
}

fn prompt_packet_count() -> Result<u32, Box<dyn Error>> {
    let packet_count = Input::new()
        .with_prompt("Enter number of packets")
        .default(3)
        .interact_text()
        .map_err(|e| format!("Failed to read packet count: {e}"))?;
    Ok(packet_count)
}

fn prompt_packet_size() -> Result<u32, Box<dyn Error>> {
    let size = Input::new()
        .with_prompt("Enter packet size (bytes)")
        .default(48)
        .interact_text()
        .map_err(|e| format!("Failed to read packet size: {e}"))?;
    Ok(size)
}

fn prompt_first_hop() -> Result<u32, Box<dyn Error>> {

}

fn prompt_max_hop() -> Result<u32, Box<dyn Error>> {

}

fn prompt_paris() -> Result<u32, Box<dyn Error> {

}

fn prompt_dont_fragment() -> Result<bool, Box<dyn Error>> {

}

fn prompt_port() -> Result<u16, Box<dyn Error>> {

}
