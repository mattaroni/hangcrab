struct SecretLetter { letter: char, hidden: bool }

impl SecretLetter {
    fn new(letter: char) -> Self {
        Self { letter, hidden: true }
    }

    fn check(&mut self, letter: char) -> bool {
        if self.hidden && letter == self.letter {
            self.hidden = false;
            return true;
        }

        return false;
    }

    fn to_char(&self) -> char {
        if self.hidden {
            return self.letter;
        }

        return '_';
    }
}

struct SecretWord { word: String, slots: Vec<SecretLetter> }

impl ToString for SecretWord {
    fn to_string(&self) -> String {
        return self.slots.iter().map(|slot| slot.to_char()).collect();
    }
}

impl SecretWord {
    fn new(word: String) -> Self {
        let slots = word.chars().map(|letter| SecretLetter::new(letter)).collect();
        Self { word, slots }
    }

    fn check_letter(&mut self, letter: char) -> usize {
        let mut count = 0;

        for slot in self.slots.iter_mut() {
            if slot.check(letter) {
                count += 1;
            }
        }

        return count;
    }

    fn check_word(&mut self, word: String) -> bool {
        let is_match = word == self.word;

        if is_match {
            for slot in self.slots.iter_mut() {
                slot.hidden = false;
            }
        }

        return is_match;
    }

    fn hidden(&self) -> bool {
        return self.slots.iter().any(|letter| letter.hidden);
    }
}

fn play_hangman(secret_word: String, lives: u8) {
    let secret_word = SecretWord::new(secret_word);

    todo!()
}
