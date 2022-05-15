use std::fs;
use std::path::Path;

use crate::constant;
use crate::game_state::GameState;
use crate::infrastructure::parser::parse_number_player;

pub fn new_game() -> GameState {
    let player_count = parse_number_player(constant::PLAYER_MAX_COUNT);
    return GameState::init(player_count);
}

pub fn resume_game() -> GameState {
    let path = Path::new(constant::SAVE_FOLDER).join(constant::SAVE_FILENAME);
    let save_content =
        fs::read_to_string(path).expect("Something went wrong reading the save file");
    return serde_json::from_str(&save_content).unwrap();
}
