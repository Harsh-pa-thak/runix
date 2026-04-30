mod word_art;
mod object_art;
mod ai;
mod cache {
    pub mod local;
    pub mod shared;
}
mod library;

use clap::Parser;

#[derive(Parser)]
#[command(name = "runix", about = "Cast ASCII spells in your terminal 🧙")]
struct Cli {
    word: String,

    #[arg(long, short = 'a', alias = "a")]
    art: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.art {
        object_art::render(&cli.word);
    } else {
        word_art::render(&cli.word);
    }
}
