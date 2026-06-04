use std::{collections::HashSet, error, fmt::Display, io::{self, Write}};

type Error = Box<dyn error::Error>;

struct SecretLetter { letter: char, hidden: bool }

impl From<char> for SecretLetter {
    fn from(value: char) -> Self {
        Self { letter: value, hidden: true }
    }
}

impl SecretLetter {
    fn check(&mut self, letter: char) -> bool {
        if self.hidden && letter == self.letter {
            self.hidden = false;
            return true;
        }

        return false;
    }

    fn to_char(&self) -> char {
        if self.hidden { '_' } else { self.letter }
    }
}

struct SecretWord { word: String, slots: Vec<SecretLetter> }

impl Display for SecretWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word: String = self.slots.iter().map(|slot| slot.to_char()).collect();
        write!(f, "{word}")
    }
}

impl From<String> for SecretWord {
    fn from(value: String) -> Self {
        let slots = value.chars().map(|letter| SecretLetter::from(letter)).collect();
        Self { word: value, slots }
    }
}

impl SecretWord {
    fn check_letter(&mut self, letter: char) -> usize {
        let mut count = 0;

        for slot in self.slots.iter_mut() {
            if slot.check(letter) {
                count += 1;
            }
        }

        count
    }

    fn hidden(&self) -> bool {
        self.slots.iter().any(|letter| letter.hidden)
    }
}

enum EndingState { Win, Loss, Quit }

struct GameTracker {
    secret_word: SecretWord,
    lives: u8,
    tried_letters: String,
    guesses: HashSet<String>,
}

impl GameTracker {
    fn new(secret_word: String, lives: u8) -> Self {
        let secret_word = SecretWord::from(secret_word);
        let tried_letters = String::new();
        let guesses = HashSet::new();

        Self { secret_word, lives, tried_letters, guesses }
    }

    fn ask_for_guess(&mut self) -> Result<Option<EndingState>, Error> {
        let mut guess = String::new();

        print!("{} · {} lives", self.secret_word, self.lives);

        if !self.tried_letters.is_empty() {
            print!(" · {}", self.tried_letters);
        }

        print!("\nyour guess: ");
        io::stdout().flush()?;

        let bytes = io::stdin().read_line(&mut guess)?;

        if bytes == 0 {
            return Ok(Some(EndingState::Quit));
        }

        guess = guess.trim_end().to_ascii_lowercase();
        Ok(self.check_guess(guess))
    }

    fn check_guess(&mut self, guess: String) -> Option<EndingState> {
        if guess.is_empty() {
            println!("Please provide a guess.");
            return None;
        }

        if !guess.chars().all(|letter| letter.is_alphabetic()) {
            println!("Please guess a valid letter or word.");
            return None;
        }

        if !self.guesses.insert(guess.clone()) {
            println!("You already made this guess.");
            return None;
        };

        if guess.len() != 1 {
            return self.try_guess_word(guess);
        }

        let letter = guess.chars().last().unwrap();
        self.try_guess_letter(letter)
    }

    fn try_guess_word(&mut self, word: String) -> Option<EndingState> {
        if word == self.secret_word.word {
            return Some(EndingState::Win);
        }

        println!("Sorry, that was not the correct word.");

        if self.lives == 1 {
            return Some(EndingState::Loss);
        }

        self.lives -= 1;
        None
    }

    fn try_guess_letter(&mut self, letter: char) -> Option<EndingState> {
        let capital_letter = letter.to_ascii_uppercase();
        let count = self.secret_word.check_letter(letter);

        match count {
            0 => {
                println!("There are no letter {capital_letter}s.");
                self.lives -= 1;
            },
            1 => println!("There is 1 letter {capital_letter}."),
            x => println!("There are {x} letter {capital_letter}s."),
        }

        if !self.tried_letters.is_empty() {
            self.tried_letters.push(' ');
        }

        self.tried_letters.push(capital_letter);

        self.check_for_end()
    }

    fn check_for_end(&self) -> Option<EndingState> {
        if self.lives == 0 {
            return Some(EndingState::Loss)
        }

        if self.secret_word.hidden() {
            return None
        }

        Some(EndingState::Win)
    }
}

pub fn play_hangman(secret_word: String, lives: u8) -> Result<(), Error> {
    let secret_word_message = format!("The secret word was: {}", secret_word);
    let mut game_tracker = GameTracker::new(secret_word, lives);
    let mut ending_state: Option<EndingState> = None;

    while ending_state.is_none() {
        ending_state = game_tracker.ask_for_guess()?;
        println!();
    }

    match ending_state.unwrap() {
        EndingState::Win => println!("You win! {secret_word_message}"),
        EndingState::Loss => println!("Game over! {secret_word_message}"),
        EndingState::Quit => println!("Goodbye!"),
    }

    Ok(())
}
