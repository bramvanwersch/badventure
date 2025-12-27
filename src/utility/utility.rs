use std::{fs::File, io::Write};

use crate::utility::Config;

pub fn save_token(token: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&config.token_location)?;
    file.write_all(token.as_bytes())?;
    Ok(())
}

pub fn read_token(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(&config.token_location)?)
}
