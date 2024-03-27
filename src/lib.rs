mod words;
pub mod game_data;

use rayon::prelude::*;

use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Debug)]
pub struct Sides {
    top: [char; 3],
    right: [char; 3],
    bottom: [char; 3],
    left: [char; 3],
    all_chars: HashSet<char>,
}

impl Sides {
    pub fn new(top: [char; 3], right: [char; 3], bottom: [char; 3], left: [char; 3]) -> Self {
        Self {
            top, right, bottom, left,
            all_chars: HashSet::from([top[0], top[1], top[2], right[0], right[1], right[2], bottom[0], bottom[1], bottom[2], left[0], left[1], left[2]]),
        }
    }

    pub fn from_str(top: &str, right: &str, bottom: &str, left: &str) -> Self {
        Self::new(
            top.chars().collect::<Vec<char>>().try_into().unwrap(),
            right.chars().collect::<Vec<char>>().try_into().unwrap(),
            bottom.chars().collect::<Vec<char>>().try_into().unwrap(),
            left.chars().collect::<Vec<char>>().try_into().unwrap(),
        )
    }

    pub fn from_sides(sides: &[String; 4]) -> Self {
        Self::new(
            sides[0].chars().collect::<Vec<char>>().try_into().unwrap(),
            sides[1].chars().collect::<Vec<char>>().try_into().unwrap(),
            sides[2].chars().collect::<Vec<char>>().try_into().unwrap(),
            sides[3].chars().collect::<Vec<char>>().try_into().unwrap(),
        )
    }

    pub fn top(&self) -> [char; 3] {
        self.top
    }

    pub fn right(&self) -> [char; 3] {
        self.right
    }

    pub fn bottom(&self) -> [char; 3] {
        self.bottom
    }

    pub fn left(&self) -> [char; 3] {
        self.left
    }

    /// Gets the side based on the given letter.
    pub fn get_side(&self, letter: char) -> Option<Side> {
        if !self.all_chars.contains(&letter) {
            None
        } else if self.top.contains(&letter) {
            Some(Side::Top)
        } else if self.right.contains(&letter) {
            Some(Side::Right)
        } else if self.bottom.contains(&letter) {
            Some(Side::Bottom)
        } else if self.left.contains(&letter) {
            Some(Side::Left)
        } else {
            None
        }
    }

    /// Gets the letters for the given side.
    pub fn get_letters(&self, side: &Side) -> [char; 3] {
        match side {
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
            Side::Left => self.left,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Game {
    sides: Sides,
    used_letters: HashSet<char>,
    remaining_letters: HashSet<char>,
    previous_words: Vec<String>,
}

impl Game {
    pub fn new(sides: Sides) -> Self {
        let remaining_letters = sides.all_chars.clone();
        
        Self {
            sides,
            used_letters: HashSet::new(),
            remaining_letters,
            previous_words: Vec::new(),
        }
    }

    /// Gets the game for the current day.
    pub fn today() -> Option<Self> {
        match game_data::get_data() {
            Some(game_data) => {
                let sides = Sides::from_sides(&game_data.sides);
                Some(Self::new(sides))
            }
            None => None
        }
    }

    /// Gets the game for yesterday.
    pub fn yesterday() -> Option<Self> {
        match game_data::get_data() {
            Some(game_data) => {
                let sides = Sides::from_sides(&game_data.yesterdaysSides);
                Some(Self::new(sides))
            }
            None => None
        }
    }

    pub fn sides(&self) -> &Sides {
        &self.sides
    }

    /// Checks if the given word is valid word, given that it is possible and is in the dictionary.
    pub fn check_dict_word(&self, word: &str) -> bool {
        if !self.previous_words.is_empty()
            && self.previous_words.last().unwrap().chars().last().unwrap() != word.chars().next().unwrap() {
            return false;
        }

        let mut prev_side: Option<Side> = None;

        for letter in word.chars() {
            if let Some(side) = self.sides.get_side(letter) {
                if let Some(prev_side) = prev_side {
                    if self.sides.get_letters(&prev_side).contains(&letter) {
                        return false;
                    }
                }
                prev_side = Some(side);
            } else {
                return false;
            }
        }

        true
    }

    /// Gets all possible words for the given game state.
    pub fn possible_words(&self) -> HashSet<String> {
        words::WORDS.iter()
            .filter_map(|w| if self.check_dict_word(w) { Some(w.to_string()) } else { None })
            .collect()
    }

    /// Gets the number of new letters that a word will add.
    fn count_new_letters(&self, word: &str) -> usize {
        word.chars().collect::<HashSet<char>>()
            .difference(&self.remaining_letters).count()
    }

    /// Plays the given word.
    pub fn play_word(&mut self, word: &str) -> usize {
        for letter in word.chars() {
            self.used_letters.insert(letter);
            self.remaining_letters.remove(&letter);
        }
        self.previous_words.push(word.to_string());

        self.count_new_letters(word)
    }

    /// Checks if the game has been won.
    pub fn check_win(&self) -> bool {
        self.used_letters.len() == 3 * 4
    }

    /// Returns a string representation of the board.
    pub fn format_board(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("  {} {} {}  \n", self.sides().top()[0], self.sides().top()[1], self.sides().top()[2]));
        output.push_str(&format!("{}       {} \n", self.sides().left()[0], self.sides().right()[0]));
        output.push_str(&format!("{}       {} \n", self.sides().left()[1], self.sides().right()[1]));
        output.push_str(&format!("{}       {} \n", self.sides().left()[2], self.sides().right()[2]));
        output.push_str(&format!("  {} {} {}  ", self.sides().bottom()[0], self.sides().bottom()[1], self.sides().bottom()[2]));

        output
    }
}

#[derive(Debug)]
pub struct Solver {}

impl Solver {
    /// Filters possible_words based on the new game state, given a possible_words list of a previous state.
    fn quick_filter_possible_words(game: &Game, possible_words: &HashSet<String>) -> HashSet<String> {
        match game.previous_words.last() {
            None => {
                possible_words.to_owned()
            }
            Some(last_word) => {
                possible_words
                    .iter()
                    .filter(|word| {
                        word.chars().next().unwrap() == last_word.chars().last().unwrap()
                    })
                    .cloned()
                    .collect()
            }
        }
    }

    /// Solves the given game, stopping when there is one or more solution with at least `min_words` words.
    /// Ex. if min_words = 2, returns when it finds a solution with two or more words.
    pub fn solve_game(game: &Game, min_words: usize) -> Vec<Vec<String>> {
        let possible_words = game.possible_words();
        let solutions = Mutex::new(Vec::new());

        let mut games = vec![game.clone()];

        for _ in 0..10 {
            if let Ok(solutions) = solutions.lock() {
                if solutions.iter().any(|s: &Vec<String>| s.len() >= min_words) {
                    break;
                }
            }

            games = games.par_iter()
                .flat_map(|game| {
                    Self::quick_filter_possible_words(game, &possible_words)
                        .iter()
                        .filter_map(|w| {
                            let mut game = game.clone();
                            game.play_word(w);
                            if game.check_win() {
                                if let Ok(mut solutions) = solutions.lock() {
                                    solutions.push(game.previous_words);
                                }
                                None
                            } else {
                                Some(game)
                            }
                        })
                        .collect::<Vec<Game>>()
                })
                .collect::<Vec<Game>>();
        }

        let mut solutions = solutions.lock().unwrap().to_owned();
        solutions.sort_by_key(|s| s.join("").len());
        solutions
    }

    /// Solves the given game, giving the most optimal solutions, sorted by total character count.
    pub fn solve(game: &Game) -> Vec<Vec<String>> {
        Self::solve_game(game, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_game() -> Game {
        let sides = Sides::from_str("DKI", "JTA", "CLV", "ERO");
        Game::new(sides)
    }

    #[test]
    fn test_mark_word() {
        let mut game = setup_game();
        assert_eq!(game.play_word("OKRA"), 4);
        assert_eq!(game.play_word("OKRA"), 0);
        assert_eq!(game.play_word("ADJECTIVAL"), 8);
    }

    #[test]
    fn test_solve() {
        let game = setup_game();
        let solutions = Solver::solve(&game);

        assert!(!solutions.is_empty());
        assert!(solutions.contains(&vec!["OKRA".to_string(), "ADJECTIVAL".to_string()]));
    }
}
