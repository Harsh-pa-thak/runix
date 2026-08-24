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
use colored::Colorize;

#[derive(Parser)]
#[command(name = "runix", about = "Cast ASCII spells in your terminal")]
struct Cli {
    #[arg(short, long)]
    art: bool,

    #[arg(short, long)]
    random: bool,

    #[arg(long)]
    set_key: Option<String>,

    #[arg(short, long)]
    list: bool,
    #[arg(short, long, default_value = "green")]
    color: String,

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

    if cli.list {
        library::print_catalog();
        return;
    }

    if cli.random {
        use rand::seq::SliceRandom;
        let spells = library::list_all();
        let mut rng = rand::thread_rng();
        if let Some(&spell) = spells.choose(&mut rng) {
            println!("{}", format!("generating a random art : '{}'", spell).green().bold());
            object_art::render(spell, &cli.color);
        }
        return;
    }

    let word = cli.word.unwrap_or_default();
    if word.trim().is_empty() {
        println!("Usage: \n
         runix <word> (print ascii)\n
         runix <word>[-a] (print art) \n
         runix -l (print list of local options)\n
         runix --set-key <KEY> (set key for AI)" );
        return;
    }

    if cli.art {
        object_art::render(&word,&cli.color);
    } else {
        word_art::render(&word);
    }
}
