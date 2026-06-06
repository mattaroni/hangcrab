use clap::Parser;

mod game;
mod wordlist;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Number of starting lives
    #[arg(short, long, default_value_t = 7)]
    lives: u8,

    /// Specify a minimum wordlength for the secret word
    #[arg(short, long)]
    min: Option<usize>,

    /// Specify a maximum wordlength for the secret word
    #[arg(short = 'M', long)]
    max: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let runner = async || {
        let secret_word = wordlist::get_random_word(args.min, args.max).await?;
        game::play_hangman(secret_word, args.lives)
    };

    if let Err(e) = runner().await {
        eprintln!("\x1b[91;1merror:\x1b[0m {e}");
    }
}
