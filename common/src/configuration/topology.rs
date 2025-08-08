use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Topology {
    mode: String,
    pub pairs: Vec<TopologicalPair>,
}

impl Topology {
    pub fn new(mode: String, pairs: Vec<TopologicalPair>) -> Self {
        Topology { mode: mode, pairs: pairs }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopologicalPair {
    from: String,
    to: String,
}

impl TopologicalPair {
    pub fn new(from: String, to: String) -> Self {
        TopologicalPair { from: from, to: to }
    }
}