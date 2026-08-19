mod word_art;
mod object_art;
mod ai;
mod cache {
    pub mod local;
    pub mod shared;
}
mod library;
mod config;

use clap::Parser;

#[derive(Parser)]
#[command(name = "runix", about = "Cast ASCII spells in your terminal 🧙")]
struct Cli {
    #[arg(short, long)]
    art: bool,

    #[arg(long)]
    set_key: Option<String>,

    word: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(key) = cli.set_key {
        match config::save_key(&key) {
            Ok(_) => println!("Key saved successfully to ~/.runix/config.json!"),
            Err(e) => eprintln!(" {}", e),
        }
        return;
    }

    let word = cli.word.unwrap_or_default();
    if word.trim().is_empty() {
        println!("Usage: runix <word> [-a] | runix --set-key <KEY>");
        return;
    }

    if cli.art {
        object_art::render(&word);
    } else {
        word_art::render(&word);
    }
}
