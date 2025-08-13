use std::{fmt::Error, fs, net::Ipv4Addr};

use common::{api::{config::Config, probes}, configuration::configuration::Configuration};
use futures::future::{join_all};
use reqwest::Client;

use clap::Parser;

mod probe;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Step 1 load configuration file into struct

    let args = Cli::parse();

    let content = fs::read_to_string(args.config)?;
    let config: Configuration = toml::from_str(&content)?;

    println!("{:?}", config);

    // Step 2 parse and process config into final requests

    let client = Client::new();

    if let Some(probe_config) = &config.probes {
        let probe_futures = probe_config
            .probes
            .iter()
            .map(|probe_id| probe::fetch_probe_information(&client, &probe_id));

        let future_results = join_all(probe_futures).await;
        println!("{:?}", future_results);
    }
    
    // Step 3 send requests and handle results

    Ok(())
}

// async fn convert_configuration(cfg: &Configuration) -> Result<Vec<Config>, Box<dyn std::error::Error>> {
//     let mut api_configurations = Vec::new();
    


//     Ok(api_configurations)
// }
