//! Layer 3: local disk cache — generate once, use forever.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn cache_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".runix").join("cache.json"))
}

fn cache_key(word: &str, mode: &str) -> String {
    format!("{}:{}", word.trim().to_lowercase(), mode)
}

pub fn get(word: &str, mode: &str) -> Option<String> {
    let path = cache_path()?;
    let data = fs::read_to_string(path).ok()?;
    let cache: HashMap<String, String> = serde_json::from_str(&data).ok()?;
    cache.get(&cache_key(word, mode)).cloned()
}

pub fn set(word: &str, mode: &str, art: &str) {
    let Some(path) = cache_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut cache: HashMap<String, String> = fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    cache.insert(cache_key(word, mode), art.to_string());

    // Compact JSON: smaller file, faster writes.
    if let Ok(json) = serde_json::to_string(&cache) {
        let _ = fs::write(&path, json);
    }
}
