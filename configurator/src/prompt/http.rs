use common::configuration::http::{HttpConfig, HttpConfigBuilder};
use dialoguer::{Input, Select};
use std::error::Error;

pub fn prompt_http_configuration() -> Result<HttpConfig, Box<dyn Error>> {
    let version = prompt_version()?;
    let port = prompt_port()?;
    let method = prompt_method()?;
    let path = prompt_path()?;
    let header_bytes = prompt_header_bytes()?;

    let http_configuration = HttpConfigBuilder::new()
        .path(path)
        .port(port)
        .method(method)
        .version(version)
        .header_bytes(header_bytes)
        .build();
    Ok(http_configuration)
}

fn prompt_path() -> Result<String, Box<dyn Error>> {
    let path = Input::new()
        .with_prompt("Enter path")
        .default("/".to_string())
        .interact_text()
        .map_err(|e| format!("Failed to read path: {e}"))?;
    Ok(path)
}

fn prompt_port() -> Result<u16, Box<dyn Error>> {
    let port = Input::new()
        .with_prompt("Enter port")
        .default(80u16)
        .validate_with(|port: &u16| -> Result<(), &str> {
            if (1..=65535).contains(port) {
                Ok(())
            } else {
                Err("Port must be between 1 and 65535")
            }
        })
        .interact_text()
        .map_err(|e| format!("Failed to read port: {e}"))?;
    Ok(port)
}

fn prompt_method() -> Result<String, Box<dyn Error>> {
    let options = vec!["GET", "POST", "PUT", "DELETE"];
    let selection = Select::new()
        .with_prompt("Choose method")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|e| format!("Failed to make method selection: {e}"))?;
    Ok(options[selection].to_string())
}

fn prompt_version() -> Result<f32, Box<dyn Error>> {
    let options = vec![1.1, 2f32];
    let selection = Select::new()
        .with_prompt("Choose http version")
        .items(&options)
        .default(0)
        .interact()
        .map_err(|e| format!("Failed to make version selection: {e}"))?;
    Ok(options[selection])
}

fn prompt_header_bytes() -> Result<u32, Box<dyn Error>> {
    let header_bytes = Input::new()
        .with_prompt("Enter amount of header bytes")
        .default(0)
        .interact_text()
        .map_err(|e| format!("Failed to read header byte amount: {e}"))?;
    Ok(header_bytes)
}
