use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Anchors {
    pub anchors: Vec<u32>,
}

impl Anchors {
    pub fn new(anchors: Vec<u32>) -> Self {
        Self { anchors }
    }
}
