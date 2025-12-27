use std::collections::HashMap;

use crate::interface::write_warning;
use crate::server_interface::{create, login};
use crate::Config;

type CommandFunction = fn(&Config) -> Result<(), Box<dyn std::error::Error>>;

pub fn parse_args(config: &Config) {
    let commands = HashMap::<String, CommandFunction>::from([
        ("login".to_string(), login as CommandFunction),
        ("create".to_string(), create as CommandFunction),
    ]);
    let pattern = std::env::args()
        .nth(1)
        .expect("Missing command, choose one of do, check or move or login");
    let result = commands.get(&pattern).ok_or(format!(
        "Invalid command '{}', choose one of {}",
        pattern,
        get_command_options(&commands)
    ));
    match result {
        Ok(func) => {
            let _ = func(config).map_err(|e| write_warning(&e.to_string()));
        }
        Err(e) => {
            write_warning(&e);
        }
    }
}

fn get_command_options(commands: &HashMap<String, CommandFunction>) -> String {
    commands
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
