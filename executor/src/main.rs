use std::panic;

use crate::api::create_measurement::create_ripe_measurement;
use clap::Parser;
use common::measurement_ids::MeasurementIds;
use futures::future::try_join_all;
use reqwest::Client;

mod api;
mod domain;
mod io;
mod transform;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let config = io::load_config(&args.config)?;
    let client = Client::new();

    let api_key = match io::prompt_api_key() {
        Ok(api_key) => api_key,
        Err(error) => panic!("{}", error),
    };

    let probe_info = api::fetch_probe_information::fetch_information(&client, &config).await?;

    if config.http_configuration.is_some() {
        if let Some(probe) = probe_info.iter().find(|probe| !probe.is_anchor) {
            panic!(
                "Probe {} is not an Anchor (required for HTTP measurements",
                probe.probe_id
            );
        }
    }

    //TODO: UPDATE THE TRANSFORM CODE TO HANDLE THE ANCHORS CORRECTLY

    let Ok(configs) = transform::generate_api_configs(config, probe_info) else {
        return Err("Could not generate API configurations".into());
    };

    let measurements = try_join_all(
        configs
            .into_iter()
            .map(|config| create_ripe_measurement(&client, config, api_key.as_str())),
    )
    .await?;

    let measurement_ids: MeasurementIds = MeasurementIds {
        ids: measurements
            .iter()
            .filter_map(|m| m.measurements.first().map(|id| id.to_string()))
            .collect::<Vec<String>>(),
    };

    match io::save_measurement_ids_to_file("measurement_ids.toml", &measurement_ids) {
        Ok(_) => println!("Measurement IDs saved successfully"),
        Err(error) => eprintln!("Failed to save measurement IDs: {}", error),
    }

    Ok(())
}
