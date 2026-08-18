use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn cache_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".runix").join("cache.json"))
}
pub fn get(word: &str, mode: &str) -> Option<String> {
    let path = cache_file_path()?;
    if !path.exists(){
        return None;
    }
    let art = fs::read_to_string(&path).ok()?;
    let cache :HashMap<String,String> = serde_json::from_str(&art).ok()?;
    let key = format!("{}:{}", word.to_lowercase(), mode);
    cache.get(&key).cloned()
}

pub fn set(word: &str, mode: &str, a: &str) {
    let Some(path)= cache_file_path() else {return};
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let mut cache :HashMap<String,String> = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|art| serde_json::from_str(&art).ok())
            .unwrap_or_default()
    } else{
        HashMap::new()
    };
    let key = format!("{}:{}", word.to_lowercase(), mode);
    cache.insert(key, a.to_string());
    if let Ok(json_str) = serde_json::to_string_pretty(&cache) {
        let _ = fs::write(&path, json_str);
    }
}
