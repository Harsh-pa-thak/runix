use colored::Colorize;

pub fn render(word: &str) {
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", art.green());
        return;
    }

    println!(
        "{}",
        format!("'{}' not in library yet — cache/AI pipeline coming in Phase 4+", word).yellow()
    );
}
