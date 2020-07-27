use serde::Deserialize;
use std::fs;
use std::io::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub directory: String,
    pub out_file: String,
    pub files: Vec<String>,
}

impl Config {
    pub fn parse(directory: String) -> Result<Self, Error> {
        let content = fs::read_to_string(directory)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}
