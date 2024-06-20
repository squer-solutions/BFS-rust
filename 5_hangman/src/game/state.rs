use std::collections::HashSet;

#[derive(Clone)]
pub struct GameState {
    pub word: String,
    pub guesses: HashSet<char>,
    pub max_errors: u8,
}

impl GameState {
    pub fn new(word: String, max_errors: u8) -> Self {
        Self {
            word,
            guesses: HashSet::new(),
            max_errors,
        }
    }

    pub fn errors(&self) -> u8 {
        self.guesses
            .iter()
            .filter(|&&c| !self.word.contains(c))
            .count() as u8
    }

    pub fn is_lost(&self) -> bool {
        self.errors() >= self.max_errors
    }

    pub fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.guesses.contains(&c))
    }
}
