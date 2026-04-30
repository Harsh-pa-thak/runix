use colored::Colorize;

pub fn render(word: &str) {
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", art.green());
        return;
    }

    println!(
        "{}",
        format!("'{}' not in library yet", word).yellow()
    );
}
