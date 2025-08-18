use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Probes {
    #[serde(rename = "type")]
    pub probe_type: String, // should always be "probes"
    pub value: String,      // A comma seperated list of probe ids
    pub requested: u32,     // The amount of comma seperated probe ids
}