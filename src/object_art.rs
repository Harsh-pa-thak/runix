use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn color_it(art: &str,color: &str)->String {
    match color.to_lowercase().as_str() {
        "red" => art.red().to_string(),
        "blue" => art.blue().to_string(),
        "cyan" => art.cyan().to_string(),
        "yellow" => art.yellow().to_string(),
        "magenta" | "purple" => art.magenta().to_string(),
        "white" => art.white().to_string(),
        _ => art.green().to_string(),
    }
}
pub fn render(word: &str,color: &str) {
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", color_it(art,color));
        return;
    }
    if let Some(art) = crate::cache::local::get(word, "object") {
        println!("{}",color_it(&art,color));
       // println!("{}", "Source: Local Cache".dimmed());
        return;
    }

    let spinner = ProgressBar::new_spinner();

    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    spinner.set_message("hold your shit runix on work");
    spinner.enable_steady_tick(Duration::from_millis(1000));

    match crate::ai::generate(word, "object") {
        Ok(art) => {
            spinner.finish_and_clear();
            crate::cache::local::set(word, "object", &art);
            println!("{}", color_it(&art, color));
        }
        Err(err) => {
            spinner.finish_and_clear();
            eprintln!("{}", "Sorry please try again".red());
            eprintln!("{}", err.to_string().red());
        }
    }
}