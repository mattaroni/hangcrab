use std::{error, fmt::Display};
use clap::Parser;

mod game;
mod wordlist;

type Error = Box<dyn error::Error>;

#[derive(Debug, Clone)]
struct ZeroLivesError;

impl Display for ZeroLivesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot start game with zero lives")
    }
}

impl error::Error for ZeroLivesError {}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Number of starting lives
    #[arg(short, long, default_value_t = 7)]
    lives: u8,

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

    if args.lives == 0 {
        return Err(Box::from(ZeroLivesError));
    }

    let secret_word = wordlist::get_random_word(args.min, args.max).await?;
    game::play_hangman(secret_word, args.lives)
}
