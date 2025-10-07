use common::{api::definitions::{DefinitionTemplate, DefinitionTemplateBuilder}, configuration::configuration::Configuration};
use futures::future::{try_join_all};
use reqwest::Client;

use clap::Parser;

use crate::probe::Probe;

mod input;
mod transform;
mod probe;
mod ripe;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config = input::load_config(&args.config)?;

    let client = Client::new();
    let probes = fetch_all_probes(&client, &config).await?;
    let connections = transform::generate_connections_from_probes(probes)?;

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

    let definitions: Vec<DefinitionTemplate> = [
            ping_definition_template,
            //TODO: add other definition types
        ].into_iter()
        .flatten()
        .collect();

    let configs = transform::create_api_configs(
        config.start_time.map(|time| time.timestamp() as u64), 
        config.end_time.map(|time| time.timestamp() as u64), 
        billed_to, connections, definitions);
    
    println!("{:?}", configs);

    //TODO: send configs
    // Step 3 send requests and handle results

    Ok(())
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