//! Code used for selecting a random word for a game of hangman.

use std::path::PathBuf;

use futures_util::StreamExt;
use rand::prelude::*;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt, BufWriter},
};

type WordlengthFilter = Result<Box<dyn Fn(&&str) -> bool>, Error>;

/// The name to use for the parent directory for the cached wordlist file
const CACHE_DIRECTORY_NAME: &str = "hangcrab";
/// The name to use for the cached wordlist file.
const WORDLIST_FILENAME: &str = "wordlist.txt";
/// The URL to the remote wordlist file.
const WORDLIST_URL: &str = "https://people.sc.fsu.edu/~jburkardt/datasets/words/sowpods.txt";

/// Errors that may occur while reading, parsing, or caching the wordlist.
#[derive(Error, Debug)]
pub enum Error {
    /// The specified wordlength minimum is greater than the specified
    /// wordlength maximum.
    #[error("wordlength minimum cannot be greater than maximum")]
    BadWordlengthRange,

    /// An IO error occured when trying to read the cached wordlist file.
    #[error("failed to read wordlist cache: {0}")]
    CacheUnreadable(io::Error),

    /// An IO error occured when attempting to cache the remote wordlist file.
    #[error("failed to cache wordlist: {0}")]
    CachingFailure(#[from] io::Error),

    /// An error occured while processing a GET request for the remote wordlist
    /// file.
    #[error("failed to read online wordlist")]
    ConnectionFailure(#[from] reqwest::Error),

    /// No word was found in the wordlist that matched the user's specified
    /// wordlength range.
    #[error("no word found matching your specifications")]
    NoWordFound,

    /// The user's operating system is not supported by this application,
    /// specifically because a caching directory could not be found for the OS'
    /// file system.
    #[error("unsupported operating system")]
    UnsupportedOS,
}

/// Returns a random word from the wordlist.
///
/// # Errors
///
/// This function will return an error if:
///
/// - The user specified a wordlength range with a minimum greater than the
///   maximum.
/// - An IO error occured while trying to read the *cached* wordlist file.
/// - The wordlist hasn't been cached yet, and an error occured while trying to
///   download the *remote* wordlist file.
/// - A dedicated caching directory (i.e. a location for storing caches) cannot
///   be found for the user's operating system.
/// - No word in the wordlist matches the user's specified wordlength range.
pub async fn get_random_word(min: Option<usize>, max: Option<usize>) -> Result<String, Error> {
    let wordlength_filter = create_wordlength_filter(min, max)?;
    let wordlist = fetch_wordlist().await?;
    let words: Vec<&str> = wordlist.split('\n').filter(wordlength_filter).collect();

    let random_word = words.choose(&mut rand::rng()).ok_or(Error::NoWordFound)?;

    Ok(random_word.to_ascii_lowercase())
}

/// Returns the contents of the wordlist file.
///
/// # Errors
///
/// This function will return an error if the wordlist could not be cached, or
/// the existing failed to be read.
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

    let wordlist = fs::read_to_string(wordlist_path).await
        .map_err(|e| Error::CacheUnreadable(e))?;

    Ok(wordlist)
}

/// Downloads the remote wordlist file as a cache.
///
/// # Errors
///
/// This function will return an error if the GET request for the remote
/// wordlist failed, or if an IO error occured when trying to write the cache.
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

/// Returns a closure that only returns true if the provided word falls within
/// the specified wordlength range.
///
/// # Errors
///
/// This function will return an error if the specified wordlength minimum is
/// greater than the wordlength maximum.
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
