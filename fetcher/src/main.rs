use clap::Parser;
use futures::future::join_all;
use reqwest::Client;

use crate::io::MeasurementSaver;

mod fetch_measurement_data;
mod io;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long)]
    measurements: String,
    #[clap(short, long, default_value = "csv")]
    output_format: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let measurement_ids = io::read_measurement_ids_from_file(&args.measurements)?;
    let client = Client::new();

    let output: Box<dyn MeasurementSaver> = match args.output_format.as_str() {
        "csv" => Box::new(io::CsvSaver::new()),
        _ => panic!("Unsupported output format"),
    };

    let futures = measurement_ids
        .ids
        .iter()
        .map(|id| fetch_measurement_data::get_measurement_data(&client, id));
    let results = join_all(futures).await;

    for result in results {
        match result {
            Ok(measurement) => println!("{:?}", measurement),
            // Ok(measurement) => output.save(&measurement)?,
            Err(error) => println!("Error: {}", error),
        }
    }

    Ok(())
}
