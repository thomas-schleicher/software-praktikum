use crate::api::fetch_probe_information::ProbeInformation;
use common::api::{
    config::Config,
    definitions::{Definition, DefinitionTemplate},
    probes::Probes,
};

//TODO: make target with sources private and make generate connections private

#[derive(Debug)]
pub struct TargetWithSources {
    pub target: String,
    pub sources: Vec<String>,
}

pub fn generate_connections_from_probes(
    mut probes: Vec<ProbeInformation>,
) -> Result<Vec<TargetWithSources>, &'static str> {
    if probes.len() < 2 {
        return Err("Not enught probes to create a connection.");
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
