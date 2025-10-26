use common::configuration::{anchors::Anchors, probes::Probes};
use dialoguer::Input;
use std::error::Error;

fn prompt_ids(prompt: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut ids = Vec::new();

    loop {
        let input: String = Input::new()
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text()?;

        if input.trim().is_empty() {
            break;
        }

        let id: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid ID, please enter a valid number.");
                continue;
            }
        };

        if ids.contains(&id) {
            println!("ID already entered. Skipping.");
            continue;
        }

        ids.push(id);
    }

    Ok(ids)
}

pub fn prompt_anchors() -> Result<Anchors, Box<dyn Error>> {
    let anchor_ids = prompt_ids("Enter a anchor ID (or leave blank to finish)")?;
    let anchors = Anchors::new(anchor_ids);
    Ok(anchors)
}

pub fn prompt_probes() -> Result<Probes, Box<dyn Error>> {
    let probe_ids = prompt_ids("Enter a probe ID (or leave blank to finish)")?;
    let probes = Probes::new(probe_ids);
    Ok(probes)
}
