use crate::Config;
use std::collections::HashMap;

pub fn login(
    username: &str,
    password: &str,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} with {}", username, password);
    let client = reqwest::blocking::Client::new();
    let data = format!("username:{}\npassword:{}", username, password);
    let response: HashMap<String, String> = client
        .post(format!("{}/login", config.server))
        .body(data)
        .send()?
        .json()?;
    for (key, value) in &response {
        println!("{}: {}", key, value);
    }
    Ok(())
}

pub fn create(
    username: &str,
    password: &str,
    confirm_password: &str,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if confirm_password != password {
        return Err("Password and confirm password do not match".into());
    }
    let client = reqwest::blocking::Client::new();
    let data = format!("username:{}\npassword:{}", username, password);
    let response: HashMap<String, String> = client
        .post(format!("{}/create", config.server))
        .body(data)
        .send()?
        .json()?;
    for (key, value) in &response {
        println!("{}: {}", key, value);
    }
    Ok(())
}
