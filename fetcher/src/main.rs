use reqwest::Client;

mod fetch_measurement_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let measurement =
        match fetch_measurement_data::get_measurement_data(&client, "", "121060141").await {
            Ok(measurement) => measurement,
            Err(error) => panic!("{:?}", error),
        };

    println!("{:#?}", measurement);

    Ok(())
}
