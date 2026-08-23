use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn render(word: &str) {
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", art.green());
        return;
    }
    if let Some(art) = crate::cache::local::get(word, "object") {
        println!("{}", art.green());
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
            println!("{}", art.green());
        }
        Err(err) => {
            spinner.finish_and_clear();
            eprintln!("{}", "Sorry please try again".red());
            eprintln!("{}", err.to_string().red());
        }
    }
}