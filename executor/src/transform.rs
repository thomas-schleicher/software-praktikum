use crate::api::fetch_probe_information::ProbeInformation;
use crate::domain::config::Config;
use crate::domain::definition::{DefinitionTemplate, HttpDefinition, PingDefinition};
use crate::domain::probes::Probes;
use common::configuration::configuration::Configuration;
use common::configuration::topology::Topology;
#[derive(Debug)]
struct TargetWithSources {
    pub target: String,
    pub sources: Vec<String>,
}

pub fn generate_api_configs(
    measurement_configuration: Configuration,
    billed_to: &str,
    probe_information: Vec<ProbeInformation>,
) -> Result<Vec<Config>, &'static str> {
    let definitions_templates =
        match create_definition_templates_from_configuration(&measurement_configuration) {
            Ok(definitions) => definitions,
            Err(e) => return Err(e), //TODO: consider wrapping the error here
        };

    let Ok(connections) = generate_connections_from_configuration(
        probe_information,
        measurement_configuration.topology,
    ) else {
        return Err(""); //TODO: come up with better error message
    };

    let configs = create_api_configs(
        measurement_configuration
            .start_time
            .map(|time| time.timestamp() as u64),
        measurement_configuration
            .end_time
            .map(|time| time.timestamp() as u64),
        billed_to,
        connections,
        definitions_templates,
    );

    Ok(configs)
}

fn create_definition_templates_from_configuration(
    config: &Configuration,
) -> Result<Vec<DefinitionTemplate>, &'static str> {
    let mut templates: Vec<DefinitionTemplate> = Vec::new();

    if let Some(ping_config) = &config.ping_configuration {
        let ping_template = PingDefinition::template()
            .packets(ping_config.packet_count)
            .size(ping_config.size)
            .interval(config.interval);
        templates.push(DefinitionTemplate::Ping(ping_template));
    }

    if let Some(http_config) = &config.http_configuration {
        let https_template = HttpDefinition::template();
        //TODO: add config parameters for http measurements
        templates.push(DefinitionTemplate::Http(https_template));
    }

    if templates.is_empty() {
        return Err("No definition templates provided in configuration.");
    }

    Ok(templates)
}

fn generate_connections_from_configuration(
    probes: Vec<ProbeInformation>,
    topology: Option<Topology>,
) -> Result<Vec<TargetWithSources>, &'static str> {
    if probes.len() < 2 {
        return Err("Not enough probes to create a connection.");
    }

    let mode = topology
        .as_ref()
        .map(|topology| topology.mode.as_str())
        .unwrap_or("all-to-all");

    match mode {
        "all-to-all" => {
            let connections = build_all_to_all_connections(&probes);
            Ok(connections)
        }
        "custom" => {
            todo!("Not implemented yet"); //TODO
        }
        _ => Err("Invalid topology mode. Cannot build connections."),
    }
}

fn build_all_to_all_connections(probes: &Vec<ProbeInformation>) -> Vec<TargetWithSources> {
    let mut configurations = Vec::with_capacity(probes.len());

    for (i, target_probe) in probes
        .iter()
        .enumerate()
        .take(probes.len().saturating_sub(1))
    {
        let sources: Vec<String> = probes
            .iter()
            .enumerate()
            .filter(|(j, _)| j > &i)
            .map(|(_, probe)| probe.address_v4.clone())
            .collect();

        configurations.push(TargetWithSources {
            target: target_probe.probe_id.to_string(),
            sources,
        })
    }

    configurations

    // OLD IMPL IN CASE THE NEW ONE FAILS //TODO: REMOVE IF DONE
    // let mut configurations = Vec::new();
    //
    // while let Some(target_probe) = probes.pop() {
    //     if probes.is_empty() {
    //         break;
    //     }
    //     let sources: Vec<String> = probes
    //         .iter()
    //         .map(|probe| probe.probe_id.to_string())
    //         .collect();
    //
    //     configurations.push(TargetWithSources {
    //         target: target_probe.probe_id.to_string(),
    //         sources,
    //     });
    // }
    //
    // Ok(configurations)
}

fn create_api_configs(
    start_time: Option<u64>,
    end_time: Option<u64>,
    billed_to: &str,
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
                .map(|definition_template| {
                    definition_template.with_target(connection.target.as_str())
                })
                .collect();

            Config {
                start_time,
                end_time,
                is_oneoff: end_time.is_none(), //expect end time none if oneoff measurement
                billed_to: billed_to.to_string(),
                probes,
                definitions,
            }
        })
        .collect()
}
