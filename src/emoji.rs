use std::fmt;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Emoji {
    pub name: String,
    pub ch: char,
    pub keywords: Vec<String>,
}

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Emoji<| name: \"{}\", ch: \"{}\", keywords: \"{:?}\" |>", self.name, self.ch, self.keywords)
    }
}

pub fn load_emoji_data(filename: &str) -> Vec<Emoji> {
    let j = fs::read_to_string(filename).expect("Could not read emoji.json file to string");
    let emojis: Vec<Emoji> = serde_json::from_str(&j).expect("Could not deserialize JSON emoji data");
    emojis
}
