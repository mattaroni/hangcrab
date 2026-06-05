use std::{error::Error, path::PathBuf};

use futures_util::StreamExt;
use rand::prelude::*;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

const CACHE_DIRECTORY_NAME: &str = "hangcrab";
const WORDLIST_FILENAME: &str = "wordlist.txt";
const WORDLIST_URL: &str = "https://www.mit.edu/~ecprice/wordlist.10000";

pub async fn get_random_word(min_length: usize, max_length: usize) -> Result<String, String> {
    let wordlist = fetch_wordlist().await?;
    let words: Vec<&str> = wordlist.split('\n')
        .filter(|word| word.len() >= min_length && word.len() <= max_length)
        .collect();

    let random_word = words.choose(&mut rand::rng()).unwrap();
    Ok(random_word.to_string())
}

async fn fetch_wordlist() -> Result<String, String> {
    let mut wordlist_path = match dirs::cache_dir() {
        Some(x) => x,
        None => return Err("unsupported operating system".to_string()),
    };

    wordlist_path.push(CACHE_DIRECTORY_NAME);
    wordlist_path.push(WORDLIST_FILENAME);

    if !wordlist_path.exists() {
        download_wordlist(&wordlist_path).await.map_err(|e| e.to_string())?;
    }

    let wordlist = fs::read_to_string(wordlist_path).await.map_err(|e| e.to_string())?;
    Ok(wordlist)
}

async fn download_wordlist(filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
    // [NOTE]: `filepath` is guarenteed to have a parent directory
    let cache_directory = filepath.parent().unwrap();

    if !cache_directory.exists() {
        fs::create_dir(cache_directory).await?;
    }

    let mut file = File::create(filepath).await?;
    let mut stream = reqwest::get(WORDLIST_URL).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    Ok(())
}
