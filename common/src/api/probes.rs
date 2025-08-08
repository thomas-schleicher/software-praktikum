use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Probes {
    #[serde(rename = "type")]
    probe_type: String, // should always be "probes"
    value: String,      // A comma seperated list of probe ids
    requested: u32,     // The amount of comma seperated probe ids
}