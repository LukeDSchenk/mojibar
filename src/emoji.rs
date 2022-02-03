use std::fmt;
use std::fs;
use serde::Deserialize;

/// This loads the emoji JSON string at compile, time, allowing it to work
/// as a WASM app (reading a file to a string doesn't work there).
/// This will add a lot of data to the end file size.
const EMOJI_JSON: &str = include_str!("../emojis/emoji-min.json");

/// Struct representing an Emoji and its associated data.
/// "ch" refers to the actual unicode value of the emoji.
/// "keywords" is a list of keywords to match on in the search bar.
#[derive(Debug, Deserialize)]
pub struct Emoji {
    pub name: String,
    pub ch: String,
    pub keywords: Vec<String>,
}

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Emoji<| name: \"{}\", ch: \"{}\", keywords: \"{:?}\" |>", self.name, self.ch, self.keywords)
    }
}

/// Loads emoji data from a JSON file. This is not supported in WebAssembly.
#[allow(dead_code)]
pub fn load_emojis_from_file(filename: &str) -> Vec<Emoji> {
    let j = fs::read_to_string(filename).expect("Could not read emoji.json file to string");
    let emojis: Vec<Emoji> = serde_json::from_str(&j).expect("Could not deserialize JSON emoji data");
    emojis
}

pub fn load_emoji_data() -> serde_json::Result<Vec<Emoji>> {
    serde_json::from_str(EMOJI_JSON)
}
