use common::configuration::ping::{PingConfig, PingConfigBuilder};
use dialoguer::Input;
use std::error::Error;

pub fn prompt_ping_config() -> Result<PingConfig, Box<dyn Error>> {
    let packet_count = prompt_packet_count()?;
    let size = prompt_packet_size()?;

    let ping_config = PingConfigBuilder::new()
        .packet_count(packet_count)
        .size(size)
        .build();
    Ok(ping_config)
}

fn prompt_packet_count() -> Result<u32, Box<dyn Error>> {
    let packet_count = Input::new()
        .with_prompt("Enter number of ping packets")
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
