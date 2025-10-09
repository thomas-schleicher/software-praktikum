use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Probes {
    #[serde(rename = "type")]
    pub probe_type: String, // should always be "probes"
    pub value: String,    // A comma seperated list of probe ids
    pub requested: usize, // The amount of comma seperated probe ids
}
