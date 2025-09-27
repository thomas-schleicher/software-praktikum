use serde::{Deserialize, Serialize};
use crate::api::{definitions::Definitions, probes::Probes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    pub is_oneoff: bool, //TODO
    pub billed_to: String, //TODO: at executor
    pub definitions: Vec<Definitions>,
    pub probes: Vec<Probes>,
}