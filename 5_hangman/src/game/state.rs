use std::collections::HashSet;

#[derive(Clone)]
pub struct GameState {
    pub word: String,
    pub guesses: HashSet<char>,
    pub max_errors: u8,
}

impl GameState {
    pub fn new(word: String, max_errors: u8) -> Self {
        let word = word.to_lowercase();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_errors() {
        let state = GameState {
            word: "hello".to_string(),
            guesses: HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e']),
            max_errors: 5,
        };

        assert_eq!(state.errors(), 4);
    }

    #[test]
    fn test_is_lost() {
        let state = GameState {
            word: "hello".to_string(),
            guesses: HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e']),
            max_errors: 3,
        };

        assert!(state.is_lost());
    }

    #[test]
    fn test_is_won() {
        let state = GameState {
            word: "hello".to_string(),
            guesses: HashSet::from_iter(vec!['h', 'e', 'l', 'o']),
            max_errors: 5,
        };

        assert!(state.is_won());
    }
}
