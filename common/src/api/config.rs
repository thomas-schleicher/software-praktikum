use serde::{Deserialize, Serialize};
use crate::api::{definitions::Definitions, probes::Probes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub is_oneoff: bool, //TODO
    pub billed_to: String, //TODO: at executor
    pub definitions: Vec<Definitions>,
    pub probes: Vec<Probes>,
}