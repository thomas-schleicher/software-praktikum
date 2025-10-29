use crate::domain::definition::Definition;
use crate::domain::probes::Probes;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<u64>,
    pub is_oneoff: bool,
    pub definitions: Vec<Definition>,
    pub probes: Vec<Probes>,
}
