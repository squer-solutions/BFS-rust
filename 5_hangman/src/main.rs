use std::error::Error;

use game::state::GameState;
use renderer::Renderer;

pub mod game;
mod renderer;

fn main() {
    let word = match get_word() {
        Ok(w) => w,
        Err(_) => {
            println!("Failed to get word");
            return;
        }
    };

    let renderer = Renderer::new();
    play_game(word, 8, &renderer);
    renderer.stop();
}

fn play_game(word: String, max_errors: u8, renderer: &Renderer) {
    let mut state = GameState::new(word, max_errors);

    while !state.is_lost() && !state.is_won() {
        renderer.update(state.clone());

        let guess = get_guess();
        state.guesses.insert(guess);
    }
    renderer.update(state.clone());
}

fn get_guess() -> char {
    loop {
        println!("Enter a guess:");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();

        if let Some(c) = guess.chars().next() {
            if c.is_alphanumeric() {
                return c;
            }
        }
    }
}

fn get_word() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://random-word.ryanrk.com/api/en/word/random")?;

    let mut word: Vec<String> = resp.json()?;

    word.pop().ok_or(Box::from("No word found"))
}
