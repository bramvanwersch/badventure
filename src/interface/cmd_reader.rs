use crate::server_interface::{create, login};
use crate::Config;

pub fn parse_args(config: &Config) {
    let pattern = std::env::args()
        .nth(1)
        .expect("Missing command, choose one of do, check or move or login");
    match pattern.as_str() {
        "do" => {
            println!("WE doing");
        }
        "check" => {
            println!("WE checking");
        }
        "move" => {
            println!("WE moving");
        }
        "login" => {
            let username = std::env::args().nth(2).expect("Expected a username");
            let password = std::env::args().nth(3).expect("Expected a password");
            login(username.as_str(), password.as_str(), config).unwrap();
        }
        "create" => {
            let username = std::env::args().nth(2).expect("Expected a username");
            let password = std::env::args().nth(3).expect("Expected a password");
            let password_repeat = std::env::args().nth(4).expect("Expected a repeat password");
            create(
                username.as_str(),
                password.as_str(),
                password_repeat.as_str(),
                config,
            )
            .unwrap();
        }
        _ => {
            println!("Invalid command, choose one of; do, check or move or login");
        }
    }
}
