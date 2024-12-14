use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub start_block: u64,
    pub checkpoints: Vec<u64>,
    pub mongo_url: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let config_data = fs::read_to_string(file_path).expect("Error reading config file");
        serde_json::from_str(&config_data).expect("Error parsing config file")
    }
}
