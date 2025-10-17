use reqwest::Client;

mod fetch_measurement_data;
mod io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    //TODO: read measurement config file path from command line argument using clap

    let measurement_ids = match io::read_measurement_ids_from_file("") {
        Ok(ids) => ids,
        Err(error) => panic!("Error: {}", error),
    };

    let measurements =
        match fetch_measurement_data::get_measurement_data(&client, "121060141").await {
            Ok(measurement) => measurement,
            Err(error) => panic!("Error: {}", error),
        };

    println!("{:#?}", measurements);

    Ok(())
}
