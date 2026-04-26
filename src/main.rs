use clap::Parser;
#[derive(Parser)]
#[command(name = "runix")]
#[command(about = "Terminal Is Looking Good")]
struct Cli {
    word: String,

    #[arg(long, default_value = "false")]
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
