use std::error;
use clap::Parser;

mod wordlist;

type Error = Box<dyn error::Error>;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Number of starting lives
    #[arg(short, long, default_value_t = 7)]
    lives: usize,

    /// Minimum wordlength
    #[arg(short, long, default_value_t = 5)]
    min: usize,

    /// Maximum wordlength
    #[arg(short = 'M', long, default_value_t = 12)]
    max: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    println!("range: {}-{}", args.min, args.max);

    let secret_word = wordlist::get_random_word(args.min, args.max).await?;
    println!("random word: {secret_word}");
    Ok(())
}
