use colored::Colorize;
use figlet_rs::FIGfont;

pub fn render(word: &str) {
    if word.is_empty() {
        eprintln!("{}", "Error: No word provided.".red());
        return;
    }

    let font = FIGfont::standard().expect("Built-in standard font should always load");

    match font.convert(word) {
        Some(figure) => {
            for line in figure.to_string().lines() {
                println!("{}", line.white().bold());
            }
        }
        None => {
            eprintln!("{}", format!("Could not render: {}", word).red());
        }
    }
}
