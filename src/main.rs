use clap::Parser;

mod game;
mod wordlist;

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
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    if args.lives == 0 {
        return Err("Cannot start with 0 lives".to_string());
    }

    let secret_word = wordlist::get_random_word(args.min, args.max).await?;
    game::play_hangman(secret_word, args.lives)
}
