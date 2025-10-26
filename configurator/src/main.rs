use common::configuration::configuration::{ConfigBuilder, Configuration};
use std::{error::Error, fs};

use crate::prompt::{general::MeasurementType, topology::TopologyMode};

mod prompt;

fn main() {
    let mut builder = ConfigBuilder::new();

    let measurement_types = match prompt::general::prompt_measurement_types() {
        Ok(types) => types,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    if let Some(start_time) = prompt::general::prompt_start_time() {
        builder = builder.start_time(start_time);
    }

    if let Some(end_time) = prompt::general::prompt_end_time() {
        builder = builder.end_time(end_time);

        match prompt::general::prompt_interval() {
            Ok(interval) => builder = builder.interval(interval),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    // Prompts for measurement specifics

    for measurement_type in &measurement_types {
        match measurement_type {
            MeasurementType::Ping => {
                let ping_config = match prompt::ping::prompt_ping_config() {
                    Ok(ping_config) => ping_config,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                };
                builder = builder.ping_configuration(ping_config);
            }
            MeasurementType::Http => {
                let http_config = match prompt::http::prompt_http_configuration() {
                    Ok(http_config) => http_config,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                };
                builder = builder.http_configuration(http_config);
            }
        }
    }

    // Topology

    let topology_mode = match prompt::topology::prompt_topology_mode() {
        Ok(topology_mode) => {
            builder = builder.mode(topology_mode.as_str());
            topology_mode
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    builder = match apply_appropriete_ids_for(builder, topology_mode, &measurement_types) {
        Ok(builder) => builder,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let config = builder.build().unwrap();
    save_config_to_file(&config, "config.toml").expect("Failed to write config");
}

fn apply_appropriete_ids_for(
    builder: ConfigBuilder,
    topology: TopologyMode,
    measurement_types: &[MeasurementType],
) -> Result<ConfigBuilder, Box<dyn Error>> {
    let use_anchor: bool = measurement_types.contains(&MeasurementType::Http);

    let builder = match (topology, use_anchor) {
        (TopologyMode::AllToAll, false) => builder.probes(prompt::probe::prompt_probes()?),
        (TopologyMode::AllToAll, true) => builder.anchors(prompt::probe::prompt_anchors()?),
        (TopologyMode::CustomPairs, _) => {
            todo!("Implement Custom Pairs");
        }
    };

    Ok(builder)
}

fn save_config_to_file(config: &Configuration, path: &str) -> std::io::Result<()> {
    let toml_str = toml::to_string_pretty(config).expect("Failed to serialize to TOML");
    fs::write(path, toml_str)?;
    Ok(())
}
