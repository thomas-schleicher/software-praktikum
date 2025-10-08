use crate::api::fetch_probe_information::ProbeInformation;
use common::api::definitions::DefinitionTemplateBuilder;
use common::api::{
    config::Config,
    definitions::{Definition, DefinitionTemplate},
    probes::Probes,
};
use common::configuration::configuration::Configuration;
//TODO: continue to refactor this away from main
//TODO: make target with sources private and make generate connections private

#[derive(Debug)]
pub struct TargetWithSources {
    pub target: String,
    pub sources: Vec<String>,
}

pub fn generate_api_configs(
    measurement_configuration: Configuration,
    billed_to: String,
    probe_information: Vec<ProbeInformation>,
) -> Result<Vec<Config>, String> {
    //TODO: add better errors

    // let definitions = create_definition_templates_from_configuration(&measurement_configuration);

    // let connections = generate_connections_from_probes(probe_information);

    // let configs = transform::create_api_configs(
    //     config.start_time.map(|time| time.timestamp() as u64),
    //     config.end_time.map(|time| time.timestamp() as u64),
    //     billed_to,
    //     connections,
    //     definitions,
    // );
    Ok(configs)
}

//TODO: better error handling + change definition template to handle multiple kinds of definition types
fn create_definition_templates_from_configuration(
    config: &Configuration,
) -> Result<Vec<DefinitionTemplate>, String> {
    //TODO: give builder better error handling in the form of custom errors
    let mut templates: Vec<DefinitionTemplate> = Vec::new();

    if let Some(ping_config) = &config.ping_configuration {
        let ping_template = DefinitionTemplateBuilder::new()
            .def_type("ping")
            .packets(ping_config.packet_count)
            .size(ping_config.size)
            .interval(config.interval)
            .build()
            .unwrap(); //TODO: update builder to use an error and propagate error with ?
        templates.push(ping_template);
    }

    if let Some(http_config) = &config.http_configuration {
        let http_template = DefinitionTemplateBuilder::new().build().unwrap(); //TODO: update builder to allow for http templates
        templates.push(http_template);
    }

    Ok(templates)
}

pub fn generate_connections_from_probes(
    mut probes: Vec<ProbeInformation>,
) -> Result<Vec<TargetWithSources>, &'static str> {
    if probes.len() < 2 {
        return Err("Not enough probes to create a connection.");
    }

    let mut configurations = Vec::new();

    while let Some(target_probe) = probes.pop() {
        if probes.is_empty() {
            break;
        }
        let sources: Vec<String> = probes
            .iter()
            .map(|probe| probe.probe_id.to_string())
            .collect();

        configurations.push(TargetWithSources {
            target: target_probe.probe_id.to_string(),
            sources,
        });
    }

    Ok(configurations)
}

pub fn create_api_configs(
    start_time: Option<u64>,
    end_time: Option<u64>,
    billed_to: String,
    connections: Vec<TargetWithSources>,
    definition_templates: Vec<DefinitionTemplate>,
) -> Vec<Config> {
    connections
        .iter()
        .map(|connection| {
            let probes: Vec<Probes> = vec![Probes {
                probe_type: "probes".to_string(),
                value: connection.sources.join(","),
                requested: connection.sources.len(),
            }];

            let definitions: Vec<Definition> = definition_templates
                .iter()
                .map(|template| template.to_definition(connection.target.clone()))
                .collect();

            Config {
                start_time,
                end_time,
                is_oneoff: end_time.is_none(), //expect end time none if oneoff measurement
                billed_to: billed_to.clone(),
                probes,
                definitions,
            }
        })
        .collect()
}
