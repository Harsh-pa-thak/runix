//! runix — cast ASCII spells in your terminal 🧙‍♂️

mod ai;
mod cache {
    pub mod local;
    pub mod shared;
}
mod config;
mod fit;
mod library;
mod pipeline;
mod render;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "runix", version, about = "Cast ASCII spells in your terminal 🧙‍♂️")]
struct Cli {

    #[arg(short, long)]
    art: bool,


    #[arg(short, long)]
    random: bool,


    #[arg(long, value_name = "KEY")]
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
            Ok(()) => println!("{}", "Key saved to ~/.runix/config.json".green()),
            Err(e) => eprintln!("{}", format!("could not save key: {e:#}").red()),
        }
        return;
    }

    if cli.list {
        library::print_catalog();
        return;
    }

    if cli.random {
        let spells = library::list_all();
        let spell = spells[rand::random::<usize>() % spells.len()];
        println!("{}", format!("Random spell: '{spell}'").green().bold());
        render::object(spell, &cli.color);
        return;
    }

    match cli.word.as_deref().map(str::trim).filter(|w| !w.is_empty()) {
        Some(word) if cli.art => render::object(word, &cli.color),
        Some(word) => render::word(word),
        None => usage(),
    }
}

fn usage() {
    println!("{}", "runix — cast ASCII spells in your terminal".green().bold());
    println!();
    println!("  runix <word>            big letter word art");
    println!("  runix <word> -a         object art (library + AI)");
    println!("  runix <word> -a -c red  object art in color");
    println!("  runix -r                random spell");
    println!("  runix -l                list built-in spells");
    println!("  runix --set-key <KEY>   save your Groq API key");
}
