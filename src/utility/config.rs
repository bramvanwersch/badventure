use std::fs;

pub struct Config {
    pub server: String,
}

impl Config {
    pub fn new(filepath: &str) -> Config {
        let data = fs::read_to_string(filepath).expect("Cannot locate config file");
        let mut server: String = "Unknown".to_string();
        for line in data.lines() {
            if let Some((key, value)) = line.split_once(":") {
                match key {
                    "server" => server = value.to_string(),
                    "bladiela" => server = value.to_string(),
                    _ => (),
                }
            }
        }
        Config { server }
    }
}
