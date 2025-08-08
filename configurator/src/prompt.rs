use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use common::configuration::ping::{PingConfig, PingConfigBuilder};
use dialoguer::{Input, MultiSelect, Select};

pub fn prompt_measurement_types() -> Vec<String> {
    let options = vec!["ping", "http"];
    let selected = MultiSelect::new()
        .with_prompt("Select type of measurement")
        .items(&options)
        .interact()
        .unwrap();

    selected.iter().map(|&i| options[i].to_string()).collect()
}

pub fn prompt_probe_ids() -> Vec<String> {
    let mut probes = Vec::new();

    loop {
        let id: String = Input::new()
            .with_prompt("Enter a probe ID (or leave blank to finish)")
            .allow_empty(true)
            .validate_with(validate_probe_id)
            .interact_text()
            .unwrap();

        if id.trim().is_empty() {
            break;
        }

        if probes.contains(&id) {
            println!("Probe ID is already entered. Skipping.");
            continue;
        }

        probes.push(id);
    }
    probes
}

pub fn prompt_topology_mode() -> String {
    let options = vec!["all-to-all", "custom pairs"];
    let selected = Select::new()
        .with_prompt("Select topology layout")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    options[selected].to_string()
}

pub fn prompt_start_time() -> Option<DateTime<Utc>> {
    prompt_optional_utc_datetime("Enter a start time in UTC (YYYY-MM-DD HH:MM), or leave blank for earliest possible")
}

pub fn prompt_end_time() -> Option<DateTime<Utc>> {
    prompt_optional_utc_datetime("Enter an end time in UTC (YYYY-MM-DD HH:MM), or leave blank for one-off measurement")
}

fn prompt_optional_utc_datetime(message: &str) -> Option<DateTime<Utc>> {
    let input: String = Input::new()
        .with_prompt(message)
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    match NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M") {
        Ok(naive_dt) => Some(Utc.from_utc_datetime(&naive_dt)),
        Err(e) => {
            eprintln!("Invalid format: {}. Expected 'YYYY-MM-DD HH:MM'", e);
            None
        }
    }
}

pub fn prompt_interval() -> u32 {
    Input::new()
        .with_prompt("Enter interval in seconds")
        .validate_with(validate_interval)
        .interact_text()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn prompt_ping_config() -> PingConfig {
    let packet_count: u32 = Input::new()
        .with_prompt("Enter number of ping packets")
        .default(3)
        .interact_text()
        .unwrap();

    let size: u32 = Input::new()
        .with_prompt("Enter packet size (bytes)")
        .default(48)
        .interact_text()
        .unwrap();

    PingConfigBuilder::new()
    .packet_count(packet_count)
    .size(size)
    .build()
    .expect("Failed to build PingConfig")
}

// support functions

fn validate_probe_id(input: &String) -> Result<(), String> {
    if input.trim().is_empty() {
        return Ok(());
    }
    if input.trim().chars().all(|c| c.is_ascii_digit()) {
        return Ok(());
    }
    return Err("Please enter a valid probe/anchor id.".to_string());
}

fn validate_interval(input: &String) -> Result<(), String> {
    if input.parse::<u32>().is_ok() {
        return Ok(());
    } else {
        return Err("Please enter a valid positive integer".to_string());
    }
}