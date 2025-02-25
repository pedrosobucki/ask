use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::model::Provider;
use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = ".config/ask";

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
    let file_path: String = get_config_path() + "/config.json";
    let error_msg: &str = "Failed to read config.json file!";

    let mut file: File = File::open(file_path).expect(error_msg);
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect(error_msg);

    let config: Config = serde_json::from_str(&contents).expect(error_msg);

    config
}

// retrieves the path to the config directory (initializes if not present yet)
fn get_config_path() -> String {
    let path: PathBuf = PathBuf::from(std::env::var("HOME").unwrap()).join(CONFIG_DIR);

    // creates config directory if not present
    if !path.exists() {
        std::fs::create_dir(&path).expect("Failed to create config path!");
    }

    path.to_str().unwrap().to_string()
}

// retrieves the path to the config directory (initializes if not present yet)
pub fn get_or_create_path(sub_path: &str) -> String {
    let path = get_config_path() + "/" + sub_path;
    let mut current = String::new();
    let split_path = path.split('/');
    let len = split_path.clone().count();

    for (i, part) in split_path.enumerate() {
        if part.is_empty() || i == len - 1 {
            continue;
        }
        current.push('/');
        current.push_str(part);

        let path = Path::new(&current);
        if path.is_file() {
            panic!("A file exists at {}", current);
        }
        if !path.exists() {
            fs::create_dir(&current).unwrap();
        }
    }

    path
}