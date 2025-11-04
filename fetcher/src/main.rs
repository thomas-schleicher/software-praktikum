use clap::Parser;
use futures::future::join_all;
use reqwest::Client;

mod api;
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

    let output: Box<dyn io::MeasurementSaver> = match args.output_format.as_str() {
        "csv" => Box::new(io::CsvSaver::new()),
        _ => panic!("Unsupported output format"),
    };

    let futures = measurement_ids
        .ids
        .iter()
        .map(|id| api::fetch_measurement_data::get_measurement_data(&client, id));
    let results = join_all(futures).await;

    let mut measurements: Vec<api::results::AggregatedMeasurement> =
        Vec::with_capacity(results.len());
    for result in results {
        match result {
            Ok(mut m) => measurements.append(&mut m),
            Err(error) => println!("Error: {}", error),
        }
    }

    output.save_by_type(&measurements)?;

    Ok(())
}
