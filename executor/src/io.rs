use common::{configuration::configuration::Configuration, measurement_ids::MeasurementIds};
use dialoguer::Password;
use std::{error::Error, fs};

pub fn load_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Configuration = toml::from_str(&content)?;
    Ok(config)
}

pub fn prompt_api_key() -> Result<String, &'static str> {
    let api_key = Password::new()
        .with_prompt("Enter your Ripe Atlas API key")
        .validate_with(|api_key: &String| -> Result<(), &str> {
            let trimmed_key = api_key.trim();
            if trimmed_key.is_empty() {
                return Err("Provided API key is empty");
            }

            if trimmed_key.len() != 36 {
                return Err("Length of provided API key is invalid");
            }

            if !trimmed_key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
            {
                return Err("Proper API keys may only contain alphanumeric characters and '-'");
            }
            Ok(())
        })
        .interact()
        .map_err(|_| "Invalid Ripe Atlas API key")?;

    Ok(api_key.trim().to_string())
}

pub fn save_measurement_ids_to_file(
    file_path: &str,
    measurement_ids: &MeasurementIds,
) -> Result<(), Box<dyn Error>> {
    let content = toml::to_string(measurement_ids)?;
    fs::write(file_path, content)?;
    Ok(())
}
