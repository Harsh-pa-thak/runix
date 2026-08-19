use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub groq_key: Option<String>,
}

fn config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".runix")
        .join("config.json")
}

pub fn save_key(key: &str) -> anyhow::Result<()> {
    let path = config_path();

    fs::create_dir_all(path.parent().unwrap())?;

    let config = Config {
        groq_key: Some(key.trim().to_string()),
    };

    fs::write(path, serde_json::to_string_pretty(&config)?)?;

    Ok(())
}

pub fn get_key() -> Option<String> {
    let _ = dotenvy::dotenv().ok();
    if let Ok(key) = std::env::var("GROQ_KEY") {
        if !key.trim().is_empty() {
            return Some(key);
        }
    }


    let path = config_path();

    let data = fs::read_to_string(path).ok()?;
    let config: Config = serde_json::from_str(&data).ok()?;


    config.groq_key
}