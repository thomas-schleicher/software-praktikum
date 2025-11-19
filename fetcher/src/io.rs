use csv::Writer;
use std::{collections::HashMap, error::Error, fs};

use common::measurement_ids::MeasurementIds;
use toml;

use crate::api::results::{
    AggregatedMeasurement, FlattenedHttpMeasurement, FlattenedTraceRouteMeasurement,
};

pub trait MeasurementSaver {
    fn save_by_type(&self, measurements: &[AggregatedMeasurement]) -> Result<(), Box<dyn Error>>;
    fn save_to(
        &self,
        file_name: &str,
        measurement: &[&AggregatedMeasurement],
    ) -> Result<(), Box<dyn Error>>;
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
    //TODO fix this saving
    fn save_by_type(&self, measurements: &[AggregatedMeasurement]) -> Result<(), Box<dyn Error>> {
        let mut buckets: HashMap<&str, Vec<&AggregatedMeasurement>> = HashMap::new();

        for item in measurements {
            buckets.entry(item.kind()).or_default().push(item);
        }

        for (kind, bucket) in buckets {
            let file_name = format!("{kind}.csv");
            self.save_to(&file_name, &bucket)?;
        }

        Ok(())
    }

    fn save_to(
        &self,
        file_name: &str,
        measurements: &[&AggregatedMeasurement],
    ) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(file_name)?;
        let mut writer = Writer::from_writer(file);

        for entry in measurements {
            match *entry {
                AggregatedMeasurement::Ping(p) => writer.serialize(p)?,
                AggregatedMeasurement::Http(p) => {
                    writer.serialize(FlattenedHttpMeasurement::from_http_measurement(p))?
                }
                AggregatedMeasurement::TraceRoute(t) => {
                    let rows = FlattenedTraceRouteMeasurement::from_traceroute_measurement(t);
                    for row in rows {
                        writer.serialize(row)?;
                    }
                }
            }
        }

        writer.flush()?;
        Ok(())
    }
}
