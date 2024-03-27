use reqwest::blocking;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct GameData {
    pub date: String,
    pub dictionary: Vec<String>,
    pub editor: String,
    pub editorImage: String,
    pub expiration: usize,
    pub id: usize,
    pub isFree: bool,
    pub ourSolution: Vec<String>,
    pub par: usize,
    pub printDate: String,
    pub sides: [String; 4],
    pub yesterdaysSides: [String; 4],
    pub yesterdaysSolution: Vec<String>,
}

/// Gets the game data for the current day by scraping the game website.
pub fn get_data() -> Option<GameData> {
    let response = blocking::get("https://www.nytimes.com/puzzles/letter-boxed");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let scripts_selector = scraper::Selector::parse("script[type=\"text/javascript\"]").unwrap();
    let mut scripts = document.select(&scripts_selector);

    if let Some(game_data) = scripts.find(|s| s.inner_html().contains("window.gameData")) {
        let start_len = "window.gameData = ".len();
        let game_data = &game_data.inner_html()[start_len..];
        match serde_json::from_str::<GameData>(game_data) {
            Ok(game_data) => {
                return Some(game_data);
            },
            Err(error) => {
                eprintln!("error = {:?}", error);
            }
        }
    }
    
    None
}
