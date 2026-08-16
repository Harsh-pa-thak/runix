use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
pub fn render(word: &str) {
    if let Some(art) = crate::library::lookup(word) {
        println!("{}", art.green());
        return;
    }
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    spinner.set_message("hold your shit hacker runix is on work for ");
    spinner.enable_steady_tick(Duration::from_millis(1000));

    match crate::ai::generate(word,"object") {
        Ok(art)=>{
            spinner.finish_and_clear();
            println!("{}", art);

        }Err(err)=>{
            spinner.finish_and_clear();
            eprintln!("Sorry please try again" );
        }
    }
}
