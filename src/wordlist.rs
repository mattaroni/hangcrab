use std::{error::Error, path::PathBuf};

use futures_util::StreamExt;
use rand::prelude::*;
use tokio::{fs::{self, File}, io::{AsyncWriteExt, BufWriter}};

const CACHE_DIRECTORY_NAME: &str = "hangcrab";
const WORDLIST_FILENAME: &str = "wordlist.txt";
const WORDLIST_URL: &str = "https://people.sc.fsu.edu/~jburkardt/datasets/words/sowpods.txt";

pub async fn get_random_word(min: Option<usize>, max: Option<usize>) -> Result<String, String> {
    let wordlength_filter = create_wordlength_filter(min, max)?;
    let wordlist = fetch_wordlist().await?;
    let words: Vec<&str> = wordlist.split('\n').filter(wordlength_filter).collect();

    let random_word = words.choose(&mut rand::rng())
        .ok_or("no word found matching your specifications")?;

    Ok(random_word.to_ascii_lowercase())
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

    let file = File::create(filepath).await?;
    let mut buffer = BufWriter::new(file);
    let mut stream = reqwest::get(WORDLIST_URL).await?.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        buffer.write_all(&chunk).await?;
    }

    buffer.flush().await?;

    Ok(())
}

fn create_wordlength_filter(
    minimum: Option<usize>,
    maximum: Option<usize>,
) -> Result<Box<dyn Fn(&&str) -> bool>, String> {
    let handle_min_max = |min, max| {
        if min > max {
            return Err("wordlength minimum cannot be greater than maximum".to_string());
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
