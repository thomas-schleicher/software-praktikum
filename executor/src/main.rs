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

    let api_key = match input::prompt_api_key() {
        Ok(api_key) => api_key,
        Err(error) => panic!("{}", error),
    };

    let probe_info = api::fetch_probe_information::fetch_all_probes(&client, &config).await?;

    let Ok(configs) = transform::generate_api_configs(config, probe_info) else {
        return Err("Could not generate API configurations".into());
    };

    let measurements = try_join_all(
        configs
            .into_iter()
            .map(|config| create_ripe_measurement(&client, config, api_key.as_str())),
    )
    .await?;

    println!("{:#?}", measurements);

    Ok(())
}
