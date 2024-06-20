use std::{collections::HashSet, sync::mpsc, thread::JoinHandle};

use crate::game::state::GameState;

pub struct Renderer {
    sender: mpsc::Sender<GameState>,
    handle: JoinHandle<()>,
}

impl Renderer {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let handle = std::thread::spawn(move || {
            render(receiver);
        });

        Self { sender, handle }
    }

    pub fn update(&self, state: GameState) {
        self.sender.send(state).unwrap();
    }

    pub fn stop(self) {
        drop(self.sender);
        self.handle.join().unwrap();
    }
}

fn render(receiver: mpsc::Receiver<GameState>) {
    loop {
        if let Ok(state) = receiver.recv() {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Word: {}", mask_word(&state.word, &state.guesses));
            println!("Guesses: {:?}", state.guesses);
            println!("Errors: {}", state.errors());
            if state.is_lost() {
                println!("You lost!");
                println!("The word was: {}", state.word);
                break;
            } else if state.is_won() {
                println!("You won!");
                break;
            }
        } else {
            break;
        }
    }
}

fn mask_word(word: &str, guesses: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guesses.contains(&c) { c } else { '_' })
        .collect()
}
