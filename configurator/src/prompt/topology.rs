use dialoguer::Select;
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy)]
pub enum TopologyMode {
    MeshFull,
    MeshDirectional,
    CustomPairs,
}

impl fmt::Display for TopologyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopologyMode::MeshFull => write!(f, "mesh-full"),
            TopologyMode::MeshDirectional => write!(f, "mesh-directional"),
            TopologyMode::CustomPairs => write!(f, "custom pairs"),
        }
    }
}

impl TopologyMode {
    pub fn all() -> &'static [TopologyMode] {
        &[
            TopologyMode::MeshFull,
            TopologyMode::MeshDirectional,
            TopologyMode::CustomPairs,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TopologyMode::MeshFull => "mesh-full",
            TopologyMode::MeshDirectional => "mesh-directional",
            TopologyMode::CustomPairs => "custom",
        }
    }
}

pub fn prompt_topology_mode() -> Result<TopologyMode, Box<dyn Error>> {
    let options = TopologyMode::all();
    let selected = Select::new()
        .with_prompt("Select topology layout")
        .items(options)
        .default(0)
        .interact()
        .map_err(|e| format!("Failed to select topology mode: {}", e))?;
    Ok(options[selected])
}
