use std::path::PathBuf;

use futures_util::StreamExt;
use rand::prelude::*;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt, BufWriter},
};

type WordlengthFilter = Result<Box<dyn Fn(&&str) -> bool>, Error>;

const CACHE_DIRECTORY_NAME: &str = "hangcrab";
const WORDLIST_FILENAME: &str = "wordlist.txt";
const WORDLIST_URL: &str = "https://people.sc.fsu.edu/~jburkardt/datasets/words/sowpods.txt";

#[derive(Error, Debug)]
pub enum Error {
    #[error("wordlength minimum cannot be greater than maximum")]
    BadWordlengthRange,

    #[error("failed to cache wordlist: {0}")]
    CachingFailure(#[from] io::Error),

    #[error("failed to read online wordlist")]
    ConnectionFailure(#[from] reqwest::Error),

    #[error("no word found matching your specifications")]
    NoWordFound,

    #[error("unsupported operating system")]
    UnsupportedOS,
}

pub async fn get_random_word(min: Option<usize>, max: Option<usize>) -> Result<String, Error> {
    let wordlength_filter = create_wordlength_filter(min, max)?;
    let wordlist = fetch_wordlist().await?;
    let words: Vec<&str> = wordlist.split('\n').filter(wordlength_filter).collect();

    let random_word = words.choose(&mut rand::rng()).ok_or(Error::NoWordFound)?;

    Ok(random_word.to_ascii_lowercase())
}

async fn fetch_wordlist() -> Result<String, Error> {
    let mut wordlist_path = match dirs::cache_dir() {
        Some(x) => x,
        None => return Err(Error::UnsupportedOS),
    };

    wordlist_path.push(CACHE_DIRECTORY_NAME);
    wordlist_path.push(WORDLIST_FILENAME);

    if !wordlist_path.exists() {
        download_wordlist(&wordlist_path).await?;
    }

    let wordlist = fs::read_to_string(wordlist_path).await?;
    Ok(wordlist)
}

async fn download_wordlist(filepath: &PathBuf) -> Result<(), Error> {
    // [NOTE]: `filepath` is guarenteed to have a parent directory
    let cache_directory = filepath.parent().unwrap();

    if !cache_directory.exists() {
        fs::create_dir(cache_directory).await?;
    }

    let mut stream = reqwest::get(WORDLIST_URL).await?.bytes_stream();
    let file = File::create(filepath).await?;
    let mut buffer = BufWriter::new(file);

    while let Some(item) = stream.next().await {
        let chunk = item?;
        buffer.write_all(&chunk).await?;
    }

    buffer.flush().await?;

    Ok(())
}

fn create_wordlength_filter(minimum: Option<usize>, maximum: Option<usize>) -> WordlengthFilter {
    let handle_min_max = |min, max| {
        if min > max {
            return Err(Error::BadWordlengthRange);
        }

        Ok(move |word: &&str| word.len() >= min && word.len() <= max)
    };

    match minimum {
        Some(min) => match maximum {
            Some(max) => Ok(Box::new(handle_min_max(min, max)?)),
            None => Ok(Box::new(move |word| word.len() >= min)),
        },
        None => match maximum {
            Some(max) => Ok(Box::new(move |word| word.len() <= max)),
            None => Ok(Box::new(|_| true)),
        },
    }
}
