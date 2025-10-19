use clap::Parser;
use futures::future::join_all;
use reqwest::Client;

mod fetch_measurement_data;
mod io;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long)]
    measurements: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let measurement_ids = io::read_measurement_ids_from_file(&args.measurements)?;
    let client = Client::new();

    let futures = measurement_ids
        .ids
        .iter()
        .map(|id| fetch_measurement_data::get_measurement_data(&client, id));

    let results = join_all(futures).await;

    let measurements =
        match fetch_measurement_data::get_measurement_data(&client, "121060141").await {
            Ok(measurement) => measurement,
            Err(error) => panic!("Error: {}", error),
        };

    println!("{:#?}", measurements);

    Ok(())
}
