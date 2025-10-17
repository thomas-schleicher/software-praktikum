use std::fs;

use common::measurement_ids::MeasurementIds;
use toml;

pub fn read_measurement_ids_from_file(
    file_path: &str,
) -> Result<MeasurementIds, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let measurement_ids: MeasurementIds = toml::from_str(&content)?;
    Ok(measurement_ids)
}
