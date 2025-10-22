use common::configuration::configuration::{ConfigBuilder, Configuration};
use common::configuration::probes::Probes;
use std::fs;

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

    for measurement_type in measurement_types {
        match measurement_type.as_str() {
            "ping" => {
                let ping_config = match prompt::ping::prompt_ping_config() {
                    Ok(ping_config) => ping_config,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                };
                builder = builder.ping_configuration(ping_config);
            }
            "http" => {
                let http_config = match prompt::http::prompt_http_configuration() {
                    Ok(http_config) => http_config,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                };
                builder = builder.http_configuration(http_config);
            }
            _ => {
                eprintln!("Unknown measurement type: {}", measurement_type);
                std::process::exit(1);
            }
        }
    }

    // Topology

    match prompt::topology::prompt_topology_mode() {
        Ok(topology_mode) => {
            match topology_mode.as_str() {
                "all-to-all" => match prompt::probe::prompt_probe_ids() {
                    Ok(probe_ids) => {
                        builder = builder.probes(Probes::new(probe_ids));
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                },
                "custom" => {
                    todo!("Implement Custom Pairs");
                }
                _ => {
                    eprintln!("Unknown topology mode: {}", topology_mode);
                    std::process::exit(1);
                }
            }
            builder = builder.mode(topology_mode);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let config = builder.build().unwrap();
    save_config_to_file(&config, "config.toml").expect("Failed to write config");
}

fn save_config_to_file(config: &Configuration, path: &str) -> std::io::Result<()> {
    let toml_str = toml::to_string_pretty(config).expect("Failed to serialize to TOML");
    fs::write(path, toml_str)?;
    Ok(())
}
