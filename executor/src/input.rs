use std::fs;

use common::configuration::configuration::Configuration;

pub fn load_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Configuration = toml::from_str(&content)?;
    Ok(config)
}