use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Probes {
    pub probes: Vec<u32>,
}

impl Probes {
    pub fn new(probes: Vec<u32>) -> Self {
        Probes { probes: probes }
    }
}
