use std::error::Error;

fn main() {
    let word = match get_word() {
        Ok(w) => w,
        Err(_) => {
            println!("Failed to get word");
            return;
        }
    };
    println!("Random word: {}", word);
}

fn get_word() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://random-word-api.herokuapp.com/word?lang=en")?;

    let mut word: Vec<String> = resp.json()?;

    word.pop().ok_or(Box::from("No word found"))
}
