use crate::api::fetch_probe_information::ProbeInformation;
use crate::domain::config::Config;
use crate::domain::definition::DefinitionTemplate;
use crate::domain::probes::Probes;
use crate::transform::connections::{TargetWithSources, generate_connections};
use crate::transform::templates::create_definition_templates;
use common::configuration::configuration::Configuration;

pub fn generate_api_configs(
    measurement_configuration: Configuration,
    probe_information: Vec<ProbeInformation>,
) -> Result<Vec<Config>, &'static str> {
    let definition_templates = create_definition_templates(&measurement_configuration)?;
    let connections = generate_connections(probe_information, &measurement_configuration)?;

    let start_time = measurement_configuration
        .start_time
        .map(|time| time.timestamp() as u64);
    let end_time = measurement_configuration
        .end_time
        .map(|time| time.timestamp() as u64);

    let configs = create_api_configs(start_time, end_time, connections, definition_templates);
    Ok(configs)
}

fn create_api_configs(
    start_time: Option<u64>,
    stop_time: Option<u64>,
    connections: Vec<TargetWithSources>,
    definition_templates: Vec<DefinitionTemplate>,
) -> Vec<Config> {
    connections
        .iter()
        .map(|connection| {
            let probes: Vec<Probes> = vec![Probes {
                probe_type: String::from("probes"),
                value: connection.sources.join(","),
                requested: connection.sources.len(),
            }];

            let definitions = definition_templates
                .iter()
                .map(
                    |definition_template| match (definition_template, &connection.target_fqdn) {
                        (DefinitionTemplate::Ping(_), _) => {
                            definition_template.with_target(connection.target_ipv4.as_str())
                        }
                        (DefinitionTemplate::Http(_), Some(fqdn)) => {
                            definition_template.with_target(fqdn)
                        }
                        (DefinitionTemplate::Http(_), None) => {
                            unreachable!("HTTP templates imply the existance of a fqdn to be set");
                        }
                    },
                )
                .collect();

            Config {
                start_time,
                stop_time,
                is_oneoff: stop_time.is_none(), //expect end time none if oneoff measurement
                probes,
                definitions,
            }
        })
        .collect()
}
