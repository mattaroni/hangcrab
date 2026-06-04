use std::{error, fmt::Display, path::PathBuf};
use futures_util::StreamExt;
use rand::prelude::*;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

const CACHE_DIRECTORY_NAME: &str = "hangcrab";
const WORDLIST_FILENAME: &str = "wordlist.txt";
const WORDLIST_URL: &str = "https://www.mit.edu/~ecprice/wordlist.10000";

type Error = Box<dyn error::Error>;

pub async fn get_random_word(min_length: usize, max_length: usize) -> Result<String, Error> {
    let wordlist = fetch_wordlist().await?;
    let words: Vec<&str> = wordlist.split('\n')
        .filter(|word| word.len() >= min_length && word.len() <= max_length)
        .collect();

    let random_word = words.choose(&mut rand::rng()).unwrap();
    Ok(random_word.to_string())
}

#[derive(Debug, Clone)]
struct NotSupportedError;

impl Display for NotSupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operating system not supported by appliction")
    }
}

impl error::Error for NotSupportedError {}

async fn fetch_wordlist() -> Result<String, Error> {
    let mut wordlist_path = match dirs::cache_dir() {
        Some(x) => x,
        None => return Err(Box::new(NotSupportedError)),
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

    if !cache_directory.parent().unwrap().exists() {
        println!("FOOOOOO")
    }

    if !cache_directory.exists() {
        fs::create_dir(cache_directory).await.expect("breaks here");
    }

    let mut file = File::create(filepath).await?;
    let mut stream = reqwest::get(WORDLIST_URL).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}
