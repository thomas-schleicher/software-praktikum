use csv::Writer;
use std::{error::Error, fs};

use common::measurement_ids::MeasurementIds;
use toml;

use crate::fetch_measurement_data::AggregatedMeasurement;

pub trait MeasurementSaver {
    fn save(&self, measurements: &Vec<AggregatedMeasurement>) -> Result<(), Box<dyn Error>>;
}

pub fn read_measurement_ids_from_file(file_path: &str) -> Result<MeasurementIds, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let measurement_ids: MeasurementIds = toml::from_str(&content)?;
    Ok(measurement_ids)
}

pub struct CsvSaver;

impl CsvSaver {
    pub fn new() -> Self {
        CsvSaver {}
    }
}

impl MeasurementSaver for CsvSaver {
    fn save(&self, measurements: &Vec<AggregatedMeasurement>) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create("measurements.csv")?;
        let mut writer = Writer::from_writer(file);

        for measurement in measurements {
            writer.serialize(measurement)?;
        }

        writer.flush()?;
        Ok(())
    }
}
