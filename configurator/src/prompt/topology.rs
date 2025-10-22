use dialoguer::Select;
use std::error::Error;

pub fn prompt_topology_mode() -> Result<String, Box<dyn Error>> {
    let options = vec!["all-to-all", "custom pairs"];
    let selected = Select::new()
        .with_prompt("Select topology layout")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|e| format!("Failed to select topology mode: {}", e))?;
    Ok(options[selected].to_string())
}
