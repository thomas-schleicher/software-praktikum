use std::fs;

use common::configuration::{
    configuration::{ConfigBuilder, Configuration},
    probes::Probes,
};

use crate::prompt::prompt_interval;

mod prompt;

fn main() {
    let mut builder = ConfigBuilder::new();

    // Prompts for general configuration
    let start_time = prompt::prompt_start_time();
    if let Some(start) = start_time {
        builder = builder.start_time(start);
    }

    let end_time = prompt::prompt_end_time();
    if let Some(end) = end_time {
        builder = builder.end_time(end);
    }

    if let Some(interval) = end_time.map(|_| prompt_interval()) {
        builder = builder.interval(interval);
    }

    // Prompts for measurement specifics
    let measurement_types = prompt::prompt_measurement_types();

    if measurement_types.iter().any(|s| s == "ping") {
        builder = builder.ping_configuration(prompt::prompt_ping_config());
    }

    //TODO: http config probe

    let topology_mode = prompt::prompt_topology_mode();
    builder = builder.mode(topology_mode);

    let probes = Probes::new(prompt::prompt_probe_ids());
    builder = builder.probes(probes);

    //TODO: probe for pairs if topology = custom

    let config = builder.build().unwrap();
    save_config_to_file(&config, "config.toml").expect("Failed to write config");
}

fn save_config_to_file(config: &Configuration, path: &str) -> std::io::Result<()> {
    let toml_str = toml::to_string_pretty(config).expect("Failed to serialize to TOML");
    fs::write(path, toml_str)?;
    Ok(())
}
