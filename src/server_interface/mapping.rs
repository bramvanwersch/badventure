use crate::{
    interface::{write_success, write_warning},
    server_interface::ServerRequest,
    Config,
};
use std::{fs::File, io::Write};

pub fn login(
    username: &str,
    password: &str,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = ServerRequest::new(
        "login",
        vec![("password", password), ("username", username)],
        config,
    );
    request.send("post")?;
    write_response(&request)?;
    let token = request.response_data.get("token").ok_or("Missing token")?;
    save_token(token, config)?;
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
    let mut request = ServerRequest::new(
        "create",
        vec![("password", password), ("username", username)],
        config,
    );
    request.send("post")?;
    write_response(&request)?;
    let token = request.response_data.get("token").ok_or("Missing token")?;
    save_token(token, config)?;
    Ok(())
}

fn save_token(token: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&config.token_location)?;
    file.write_all(token.as_bytes())?;
    Ok(())
}

pub fn write_response(request: &ServerRequest) -> Result<(), Box<dyn std::error::Error>> {
    let message = request
        .response_data
        .get("message")
        .ok_or("Missing message in response")?;
    if request.was_success() {
        write_success(message);
    } else {
        write_warning(message);
    }
    Ok(())
}
