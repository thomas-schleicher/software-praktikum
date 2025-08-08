use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Probes {
    pub probes: Vec<String>,
}

impl Probes {
    pub fn new(probes: Vec<String>) -> Self {
        Probes { probes: probes }
    }
}