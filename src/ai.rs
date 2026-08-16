use anyhow  :: {bail , Result};
use serde_json::json;


fn clean_ascii_art(raw: &str) -> String {
    let cleaned = raw
        .trim()
        .strip_prefix("```ascii").unwrap_or(raw)
        .strip_prefix("```").unwrap_or(raw)
        .strip_suffix("```").unwrap_or(raw)
        .trim();
    cleaned
        .lines()
        .take(12)
        .filter(|line| line.chars().all(|c| c.is_ascii()))
        .collect::<Vec<&str>>()
        .join("\n")
}
pub fn generate(_word: &str, _mode: &str) -> anyhow::Result<String> {
    anyhow::bail!("AI will come soon");
}
