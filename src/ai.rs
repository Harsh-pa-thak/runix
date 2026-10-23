//! AI fallback (Layer 4): Groq chat completions, tuned for minimum latency.

use anyhow::bail;
use serde_json::json;
use std::time::Duration;

/// `openai/gpt-oss-20b` is Groq's fastest production model (~1000 tok/s).
/// `reasoning_effort: low` is the key to fast art: at the default effort the
/// model burns its whole token budget reasoning and often returns nothing.
const MODEL: &str = "openai/gpt-oss-20b";
const REASONING_EFFORT: &str = "low";

/// Art is capped at ~14 lines x ~60 columns, so 700 tokens is generous headroom.
const MAX_COMPLETION_TOKENS: u32 = 700;

const TIMEOUT: Duration = Duration::from_secs(10);

pub fn generate(word: &str) -> anyhow::Result<String> {
    let api_key = crate::config::get_key().ok_or_else(|| {
        anyhow::anyhow!(
            "no Groq API key found\n \
             grab a free key at https://console.groq.com/keys\n  \
             then run: runix --set-key <YOUR_GROQ_KEY>"
        )
    })?;

    // Ask the AI for art that fits the current terminal.
    let width = crate::fit::art_width_budget();
    let height = crate::fit::art_height_budget();

    let client = reqwest::blocking::Client::builder()
        .timeout(TIMEOUT)
        .tcp_nodelay(true)
        .build()?;

    let body = json!({
        "model": MODEL,
        "reasoning_effort": REASONING_EFFORT,
        "temperature": 0.5,
        "max_completion_tokens": MAX_COMPLETION_TOKENS,
        "messages": [
            {
                "role": "system",
                "content": "You are a professional ASCII artist. Reply with the finished artwork and nothing else: no words, no markdown fences, no non-ASCII characters."
            },
            {
                "role": "user",
                "content": format!(
                    "Draw recognizable ASCII art of: {word}\n\
                     Hard limits (the terminal is only {width} columns by {height} rows):\n\
                     - at most {width} characters per line\n\
                     - at most {height} lines\n\
                     - make the object immediately recognizable, preserve symmetry, use varied detail"
                )
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
        let status = res.status();
        let err_text = res.text().unwrap_or_default();
        bail!("Groq API error ({}): {}", status, err_text);
    }

    let json_res: serde_json::Value = res.json()?;
    let content = json_res["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("failed to parse Groq response"))?;

    let art = clean_ascii_art(content, height);
    if art.is_empty() {
        bail!("generated art was empty or invalid");
    }

    Ok(art)
}

/// Strips accidental code fences, drops non-ASCII lines, trims blank edges
/// and caps the result to the terminal height budget.
fn clean_ascii_art(raw: &str, max_lines: usize) -> String {
    let mut s = raw.trim();
    if let Some(rest) = s.strip_prefix("```") {
        s = rest
            .strip_prefix("ascii")
            .or_else(|| rest.strip_prefix("text"))
            .unwrap_or(rest);
        if let Some(rest) = s.strip_suffix("```") {
            s = rest;
        }
    }

    let lines: Vec<&str> = s
        .lines()
        .map(str::trim_end)
        .filter(|line| line.is_ascii())
        .collect();

    let first = lines.iter().position(|l| !l.trim().is_empty()).unwrap_or(0);
    let last = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map_or(first, |i| i + 1);

    lines[first..last.min(first + max_lines)].join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_markdown_fences() {
        let raw = "```ascii\n  /\\_/\\\n ( o.o )\n  =^= \n```";
        assert_eq!(clean_ascii_art(raw, 14), "  /\\_/\\\n ( o.o )\n  =^=");
    }

    #[test]
    fn drops_non_ascii_lines() {
        let raw = "  /\\_/\\  ✨\n ( o.o )\n  =^= ";
        assert_eq!(clean_ascii_art(raw, 14), " ( o.o )\n  =^=");
    }

    #[test]
    fn caps_height() {
        assert_eq!(clean_ascii_art("a\nb\nc", 2), "a\nb");
    }

    #[test]
    fn empty_junk_is_empty() {
        assert_eq!(clean_ascii_art("```\n```", 14), "");
    }
}
