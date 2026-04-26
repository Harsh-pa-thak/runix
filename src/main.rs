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
#[command(name = "runix")]
#[command(about = "Terminal Is Looking Good")]
struct Cli {
    word: String,

    #[arg(long)]
    art: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.art {
        println!(" the word is art  {}", cli.word);
    }
    else{
        println!("the word is {}", cli.word);
    }

}
