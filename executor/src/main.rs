use crate::api::create_measurement::create_ripe_measurement;
use clap::Parser;
use futures::future::try_join_all;
use reqwest::Client;

mod api;
mod domain;
mod input;
mod transform;

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

    //TODO: handle input for this
    let billed_to = "thschleicher@edu.aau.at";
    //TODO: get api key from console
    let api_key = "Test Key";

    let probe_info = api::fetch_probe_information::fetch_all_probes(&client, &config).await?;

    let Ok(configs) = transform::generate_api_configs(config, billed_to, probe_info) else {
        return Err("Could not generate API configurations".into());
    };

    let measurements = try_join_all(
        configs
            .into_iter()
            .map(|config| create_ripe_measurement(&client, config, api_key)),
    )
    .await?;

    //TODO: output of the measurements into some format that is usable in the fetcher program

    Ok(())
}
