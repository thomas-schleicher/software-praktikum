use serde::{Deserialize, Serialize};
use crate::api::{definitions::Definitions, probes::Probes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    is_oneoff: bool, //TODO
    billed_to: String, //TODO: at executor
    definitions: Vec<Definitions>,
    probes: Vec<Probes>,
}