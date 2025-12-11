use std::collections::HashMap;

use reqwest::{blocking::RequestBuilder, StatusCode};

use crate::utility::Config;

pub struct ServerRequest<'a> {
    path: String,
    body: String,
    config: &'a Config,

    pub response_data: HashMap<String, String>,
    response_code: Option<StatusCode>,
}

impl<'a> ServerRequest<'a> {
    pub fn new(path: &str, data: Vec<(&str, &str)>, config: &'a Config) -> ServerRequest<'a> {
        // form body
        let mut body_pieces = Vec::new();
        for (name, value) in data {
            body_pieces.push(format!("{}:{}", name, value));
        }
        ServerRequest {
            path: path.to_string(),
            body: body_pieces.join("\n"),
            config,
            response_data: HashMap::new(),
            response_code: None,
        }
    }

    pub fn send(&mut self, method: &str) -> Result<(), Box<dyn std::error::Error>> {
        match method.to_lowercase().as_str() {
            "get" => {
                let client = reqwest::blocking::Client::new();
                let message = client
                    .get(format!("{}/{}", self.config.server, self.path))
                    .body(self.body.clone());
                self.send_and_process(message)?;
            }
            "post" => {
                let client = reqwest::blocking::Client::new();
                let message = client
                    .post(format!("{}/{}", self.config.server, self.path))
                    .body(self.body.clone());
                self.send_and_process(message)?;
            }
            _ => return Err("Unknown method".into()),
        }
        Ok(())
    }

    fn send_and_process(
        &mut self,
        message: RequestBuilder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = message.send()?;
        self.response_code = Some(response.status());
        let body = response.text_with_charset("utf-8")?;
        println!("{}", body);
        for line in body.lines() {
            if let Some((key, value)) = line.split_once(":") {
                self.response_data
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(())
    }

    pub fn was_success(&self) -> bool {
        if self.response_code.is_some() {
            return self.response_code.unwrap().is_success();
        }
        false
    }
}
