use std::{fs, vec};

use common::{api::{config::Config, definitions::{Definition, DefinitionTemplate, DefinitionTemplateBuilder}, probes::Probes}, configuration::configuration::Configuration};
use futures::future::{try_join_all};
use reqwest::Client;

use clap::Parser;

use crate::probe::Probe;

mod probe;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[derive(Debug)]
struct TargetWithSources {
    pub target: String,
    pub sources: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config = load_config(&args.config)?;

    let client = Client::new();
    let probes = fetch_all_probes(&client, &config).await?;
    let connections = generate_connections_from_probes(probes)?;

    let ping_definition_template= config.ping_configuration.as_ref().map(|ping_config| {
        DefinitionTemplateBuilder::new()
            .def_type("ping")
            .packets(ping_config.packet_count)
            .size(ping_config.size)
            .interval(config.interval)
            .build()
            .unwrap()
    });

    //TODO: handle input for this
    let billed_to = "thschleicher@edu.aau.at".to_string();

    //TODO: handle the defintion template creation
    let definitions = vec![ping_definition_template.unwrap()]; 

    let configs = create_api_configs(
        config.start_time.map(|time| time.timestamp() as u64), 
        config.end_time.map(|time| time.timestamp() as u64), 
        billed_to, connections, definitions);
    
    println!("{:?}", configs);

    // Step 3 send requests and handle results

    Ok(())
}

fn create_api_configs(start_time: Option<u64>, end_time: Option<u64>, billed_to: String, connections: Vec<TargetWithSources>, definition_templates: Vec<DefinitionTemplate>) -> Vec<Config> {
    connections.iter().map(|connection| {
        let probes: Vec<Probes> = vec![
            Probes { 
                probe_type: "probes".to_string(), 
                value: connection.sources.join(","),
                requested: connection.sources.len()
            }
        ];

        let definitions: Vec<Definition> = definition_templates
            .iter()
            .map(|template| template.to_definition(connection.target.clone()))
            .collect();

        Config { 
            start_time: start_time, 
            end_time: end_time, 
            is_oneoff: end_time.is_none(), //expect end time none if oneoff measurement 
            billed_to: billed_to.clone(),
            probes: probes,
            definitions: definitions
        }
    }).collect()
}

fn load_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Configuration = toml::from_str(&content)?;
    Ok(config)
}

async fn fetch_all_probes(client: &Client, config: &Configuration) -> Result<Vec<Probe>, Box<dyn std::error::Error>> {
    let Some(probe_config) = &config.probes else {
        return Err("No probes in configuration".into());
    };

    let futures = probe_config
        .probes
        .iter()
        .map(|probe_id| probe::fetch_probe_information(&client, &probe_id));

    let probes = try_join_all(futures).await?;
    Ok(probes)
}

fn generate_connections_from_probes(mut probes: Vec<Probe>) -> Result<Vec<TargetWithSources>, &'static str> {
    if probes.len() < 2 {
        return Err("Not enught probes to create a connection.");
    }
    
    let mut configurations = Vec::new();
 
    while let Some(target_probe) = probes.pop() {
        if probes.is_empty() {
            break;
        }
        let sources: Vec<String> = probes.iter()
            .map(|probe| probe.probe_id.to_string())
            .collect();
            
        configurations.push(TargetWithSources {
            target: target_probe.probe_id.to_string(),
            sources,
        });
    }

    Ok(configurations)
}