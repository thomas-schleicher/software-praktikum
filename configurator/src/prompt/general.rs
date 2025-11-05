use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dialoguer::{Input, MultiSelect};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementType {
    Ping,
    Http,
    Traceroute,
}

impl Display for MeasurementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeasurementType::Ping => write!(f, "Ping"),
            MeasurementType::Http => write!(f, "HTTP"),
            MeasurementType::Traceroute => write!(f, "Traceroute"),
        }
    }
}

impl MeasurementType {
    pub fn all() -> &'static [MeasurementType] {
        &[MeasurementType::Ping, MeasurementType::Http]
    }
}

pub fn prompt_measurement_types() -> Result<Vec<MeasurementType>, Box<dyn Error>> {
    let options = MeasurementType::all();
    let selected = MultiSelect::new()
        .with_prompt("Select type of measurement")
        .items(options)
        .interact()
        .map_err(|e| format!("Failed to select measurement type: {e}"))?;

    Ok(selected.iter().map(|&i| options[i]).collect())
}

pub fn prompt_interval() -> Result<u32, Box<dyn Error>> {
    let interval = Input::new()
        .with_prompt("Enter interval in seconds")
        .default(60)
        .validate_with(validate_interval)
        .interact_text()
        .map_err(|e| format!("Failed to enter interval: {e}"))?;

    Ok(interval)
}

fn validate_interval(input: &u32) -> Result<(), &'static str> {
    if *input > 0 {
        Ok(())
    } else {
        Err("Please enter a valid positive integer")
    }
}

pub fn prompt_start_time() -> Option<DateTime<Utc>> {
    prompt_optional_utc_datetime(
        "Enter a start time in UTC (YYYY-MM-DD HH:MM), or leave blank for earliest possible",
    )
}

pub fn prompt_end_time() -> Option<DateTime<Utc>> {
    prompt_optional_utc_datetime(
        "Enter an end time in UTC (YYYY-MM-DD HH:MM), or leave blank for one-off measurement",
    )
}

fn prompt_optional_utc_datetime(message: &str) -> Option<DateTime<Utc>> {
    let input: String = Input::new()
        .with_prompt(message)
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    match NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M") {
        Ok(naive_dt) => Some(Utc.from_utc_datetime(&naive_dt)),
        Err(e) => {
            eprintln!("Invalid format: {}. Expected 'YYYY-MM-DD HH:MM'", e);
            None
        }
    }
}
