use colored::Colorize;
use figlet_rs::FIGlet;

pub fn render(word: &str) {
    if word.is_empty() {
        eprintln!("{}", "Error: No word provided.".red());
        return;
    }

    // Load the built-in "standard" figlet font (no font file required)
    let font = FIGlet::standard().expect("Built-in standard font should always load");

    match font.convert(word) {
        Some(figure) => {
            for line in figure.to_string().lines() {
                println!("{}", line.white().bold());
            }
        }
        None => {
            // Fallback: just print the word uppercased if figlet can't render it
            eprintln!("{}", format!("Could not render: {}", word).red());
        }
    }
}
