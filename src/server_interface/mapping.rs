use crate::{
    interface::{write_success, write_warning},
    server_interface::ServerRequest,
    utility::save_token,
    Config,
};

pub fn login(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let username = std::env::args().nth(2).expect("Expected a username");
    let password = std::env::args().nth(3).expect("Expected a password");
    let mut request = ServerRequest::new(
        "login",
        vec![("password", &password), ("username", &username)],
        config,
        false,
    )?;
    request.send("post")?;
    write_response(&request)?;
    let token = request.response_data.get("token").ok_or("Missing token")?;
    save_token(token, config)?;
    Ok(())
}

pub fn create(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let username = std::env::args().nth(2).expect("Expected a username");
    let password = std::env::args().nth(3).expect("Expected a password");
    let confirm_password = std::env::args().nth(4).expect("Expected a repeat password");
    if confirm_password != password {
        return Err("Password and confirm password do not match".into());
    }
    let mut request = ServerRequest::new(
        "create",
        vec![("password", &password), ("username", &username)],
        config,
        false,
    )?;
    request.send("post")?;
    write_response(&request)?;
    let token = request.response_data.get("token").ok_or("Missing token")?;
    save_token(token, config)?;
    Ok(())
}

pub fn examine(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = ServerRequest::new("examine", vec![], config, true)?;
    request.send("post")?;
    write_response(&request)?;
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
