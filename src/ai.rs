use anyhow::bail;
use serde_json::json;

fn clean_ascii_art(raw: &str) -> String {
    let cleaned = raw
        .trim()
        .strip_prefix("```ascii")
        .or_else(|| raw.trim().strip_prefix("```"))
        .unwrap_or(raw)
        .strip_suffix("```")
        .unwrap_or(raw)
        .trim();

    cleaned
        .lines()
        .take(12)
        .filter(|line| line.chars().all(|c| c.is_ascii()))
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn generate(word: &str, _mode: &str) -> anyhow::Result<String> {
    let _ = dotenvy::dotenv();

    let api_key = std::env::var("GROQ_KEY")
        .map_err(|_| anyhow::anyhow!("GROQ_KEY not found in environment or .env file"))?;

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .build()?;

    let system_prompt = "You are a professional ASCII artist specializing in clean terminal ASCII art.\n\
                     Create a visually accurate and recognizable ASCII representation of the requested object.\n\
                     CRITICAL RULES:\n\
                     1. Output ONLY the ASCII artwork. No explanation, title, or Markdown.\n\
                     2. Never use code fences.\n\
                     3. Use spaces and ASCII characters freely to create the shape.\n\
                     4. Keep the artwork between 6 and 14 lines tall.\n\
                     5. Keep the artwork under 50 characters wide.\n\
                     6. Preserve symmetry when the object naturally has symmetry.\n\
                     7. Focus on the object's most distinctive visual features.\n\
                     8. Use multiple levels of detail rather than making a simple symbol.\n\
                     9. Make every line intentionally contribute to the shape.\n\
                     10. Do not draw a generic representation; make the requested object immediately recognizable.";
    let user_prompt = format!("Draw recognizable ASCII art of: {}", word);

    let body = json!({
   "model": "openai/gpt-oss-20b",
    "temperature": 0.4,
    "messages": [
        {
            "role": "system",
            "content": system_prompt
        },
        {
            "role": "user",
            "content": user_prompt
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