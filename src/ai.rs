use anyhow  :: {bail , Result};
use serde_json::json;
use std::env;
use dotenvy::dotenv;


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
    dotenvy::dotenv();
    let api_key = env::var("GROQ_KEY")?;
    let client = reqwest::blocking::Client::new();
    let myPrompt= format!(
        "Draw a centered ASCII art illustration of a '{}'.\n\
         Rules:\n\
         - Maximum 10-12 lines tall\n\
         - Use only standard printable ASCII characters\n\
         - Output ONLY the raw ASCII art\n\
         - Do NOT include markdown code blocks, explanation, headers, or text"
        word
    );
    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "temperature": 0.2,
        "messages": [
            {
                "role": "user",
                "content": myPrompt,
            }
        ]
    });
    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_default();
        bail!("Groq API error: {}", err_text);
    }
    let json_res: serde_json::Value = res.json()?;
    let content = json_res["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse response content from Groq"))?;
    let art = clean_ascii_art(content);
    if art.is_empty() {
        bail!("Generated art was empty or invalid");
    }
    Ok(art)
}

