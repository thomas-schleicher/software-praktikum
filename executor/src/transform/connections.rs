use crate::api::fetch_probe_information::ProbeInformation;
use common::configuration::configuration::Configuration;

#[derive(Debug)]
pub struct TargetWithSources {
    pub target_ipv4: String,
    pub target_fqdn: Option<String>,
    pub sources: Vec<String>,
}

pub fn generate_connections(
    probes: Vec<ProbeInformation>,
    configuration: &Configuration,
) -> Result<Vec<TargetWithSources>, &'static str> {
    if probes.len() < 2 {
        return Err("Not enough probes to create a connection.");
    }

    let topology_mode = configuration
        .topology
        .as_ref()
        .map(|topology| topology.mode.as_str())
        .unwrap_or("all-to-all");

    if configuration.http_configuration.is_some() && probes.iter().any(|probe| !probe.is_anchor) {
        return Err("One or multiple probes are not an Anchor (required for HTTP measurements)");
    }

    match topology_mode {
        "all-to-all" => {
            let connections = generate_all_to_all_connections(&probes);
            Ok(connections)
        }
        "custom" => {
            todo!("Not implemented yet");
        }
        _ => Err("Invalid topology mode. Cannot build connections."),
    }
}

fn generate_all_to_all_connections(probes: &Vec<ProbeInformation>) -> Vec<TargetWithSources> {
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
            .map(|(_, probe)| probe.probe_id.to_string())
            .collect();

        configurations.push(TargetWithSources {
            target_ipv4: target_probe.address_v4.to_string(),
            target_fqdn: target_probe.fqdn.clone(),
            sources,
        })
    }

    configurations
}
