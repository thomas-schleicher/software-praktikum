use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeasurementIds {
    pub ids: Vec<String>,
}

impl MeasurementIds {
    pub fn new(ids: Vec<String>) -> Self {
        MeasurementIds { ids }
    }
}
