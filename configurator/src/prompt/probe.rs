use dialoguer::Input;
use std::error::Error;

pub fn prompt_probe_ids() -> Result<Vec<String>, Box<dyn Error>> {
    let mut probes = Vec::new();

    loop {
        let id: String = Input::new()
            .with_prompt("Enter a probe ID (or leave blank to finish)")
            .allow_empty(true)
            .validate_with(validate_probe_id)
            .interact_text()
            .map_err(|e| format!("Failed to read probe ID: {}", e))?;

        if id.trim().is_empty() {
            break;
        }

        if probes.contains(&id) {
            println!("Probe ID is already entered. Skipping.");
            continue;
        }

        probes.push(id);
    }
    Ok(probes)
}

fn validate_probe_id(input: &String) -> Result<(), &'static str> {
    if input.trim().is_empty() {
        return Ok(());
    }
    if input.trim().chars().all(|c| c.is_ascii_digit()) {
        return Ok(());
    }
    Err("Please enter a valid probe/anchor id.")
}
