use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use crate::model::Provider;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub openai_api_key: String,
    pub xai_api_key: String,
    pub anthropic_api_key: String,
    pub provider: Provider,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

pub fn get_config() -> Config {
    let mut file: File = File::open("config.json").expect("Failed to read config.json file!");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config.json file!");
    let config: Config = serde_json::from_str(&contents).expect("Failed to read config.json file!");

    config
}