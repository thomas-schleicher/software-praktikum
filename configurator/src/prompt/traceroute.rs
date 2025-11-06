use common::configuration::traceroute::TracerouteConfig;
use dialoguer::{Confirm, Input, Select};
use std::error::Error;

pub fn prompt_traceroute_config() -> Result<TracerouteConfig, Box<dyn Error>> {
    let protocol = prompt_protocol()?;
    let packets = prompt_packet_count()?;
    let size = prompt_packet_size()?;
    let first_hop = prompt_first_hop()?;
    let max_hops = prompt_max_hop()?;
    let paris = prompt_paris()?;
    let dont_fragment = prompt_dont_fragment()?;

    let port = if protocol.eq_ignore_ascii_case("TCP") {
        Some(prompt_port()?)
    } else {
        None
    };

    Ok(TracerouteConfig {
        protocol,
        packets,
        size,
        first_hop,
        max_hops,
        paris,
        dont_fragment,
        port,
    })
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
    let first_hop = Input::new()
        .with_prompt("Enter first hop")
        .default(1)
        .interact_text()
        .map_err(|e| format!("Failed to read first hop: {e}"))?;
    Ok(first_hop)
}

fn prompt_max_hop() -> Result<u32, Box<dyn Error>> {
    let max_hop = Input::new()
        .with_prompt("Enter max hop count")
        .default(32)
        .interact_text()
        .map_err(|e| format!("Failed to read max hop count: {e}"))?;
    Ok(max_hop)
}

fn prompt_paris() -> Result<u32, Box<dyn Error>> {
    let paris = Input::new()
        .with_prompt("Enter paris")
        .default(16)
        .interact_text()
        .map_err(|e| format!("Failed to read paris: {e}"))?;
    Ok(paris)
}

fn prompt_dont_fragment() -> Result<bool, Box<dyn Error>> {
    let fragment = Confirm::new()
        .with_prompt("Do you wish to disable fragmenting")
        .default(false)
        .interact()
        .map_err(|e| format!("Failed to read fragmentation decision: {e}"))?;
    Ok(fragment)
}

fn prompt_port() -> Result<u16, Box<dyn Error>> {
    let port = Input::new()
        .with_prompt("Enter port")
        .default(80)
        .interact_text()
        .map_err(|e| format!("Failed to read port: {e}"))?;
    Ok(port)
}
