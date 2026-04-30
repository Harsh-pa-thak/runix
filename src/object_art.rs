use colored::Colorize;

pub fn render(word: &str) {
    // Layer 1: Common Library — instant, no network
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", art.green());
        println!("{}", "⚡ from library".dimmed());
        return;
    }

    // Layers 2–4 (local cache → shared Redis → AI) will be wired in Phases 4–6.
    println!(
        "{}",
        format!("'{}' not in library yet — cache/AI pipeline coming in Phase 4+", word).yellow()
    );
}
