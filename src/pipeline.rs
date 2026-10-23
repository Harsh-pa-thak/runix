//! The lookup pipeline every spell travels through:
//! built-in library → local cache → shared cache → AI.
//! The AI only ever runs when every cache layer misses.

use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub enum Source {
    Library,
    LocalCache,
    SharedCache,
    Ai,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::Library => "library",
            Source::LocalCache => "local cache",
            Source::SharedCache => "shared cache",
            Source::Ai => "ai",
        }
    }
}

pub struct Spell {
    pub art: String,
    pub source: Source,
    pub elapsed: Duration,
}

/// Fetch object art through the layer pipeline.
pub fn object(word: &str) -> anyhow::Result<Spell> {
    let start = Instant::now();
    let word = word.trim().to_lowercase();

    if let Some(art) = crate::library::lookup(&word) {
        return Ok(Spell {
            art: art.to_string(),
            source: Source::Library,
            elapsed: start.elapsed(),
        });
    }
    if let Some(art) = crate::cache::local::get(&word, "object") {
        return Ok(Spell {
            art,
            source: Source::LocalCache,
            elapsed: start.elapsed(),
        });
    }
    if let Some(art) = crate::cache::shared::get(&word, "object") {
        return Ok(Spell {
            art,
            source: Source::SharedCache,
            elapsed: start.elapsed(),
        });
    }

    // Last resort: the AI. Show a live loading screen while it works.
    let spinner = start_spinner(&word);
    let generated = crate::ai::generate(&word);
    spinner.finish_and_clear();

    let art = generated?;
    crate::cache::local::set(&word, "object", &art);
    Ok(Spell {
        art,
        source: Source::Ai,
        elapsed: start.elapsed(),
    })
}

/// Fast, live loading screen shown only while the AI is working.
fn start_spinner(word: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("🪄✨🌟⭐🌠✦ ")
            .template("{spinner:.yellow} {msg} {elapsed_precise}")
            .unwrap_or_else(|_| ProgressStyle::default_spinner()),
    );
    spinner.set_message(format!("casting '{word}' through the AI cauldron"));
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}
