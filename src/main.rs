use std::fs;
use std::io;
use std::path::Path;

use pickomino::GameState;

mod constant;
mod shell;

const PLAYER_MAX_COUNT: usize = 8;

fn main() {
    let mut game: GameState = parse_party_selection();
    game.run();
}

fn parse_number_player(max_player: usize) -> usize {
    println!("Please input number of players.");
    loop {
        let mut player_count = String::new();

        io::stdin()
            .read_line(&mut player_count)
            .expect("Failed to read line");

        if let Ok(o) = player_count.trim().parse::<usize>() {
            if o <= max_player {
                return o;
            }
            println!("Enter a number below {}", max_player);
            continue;
        };
        println!("Please enter valid number");
    }
}

fn parse_party_selection() -> GameState {
    loop {
        shell::print_seperator_shell();
        println!("Please select an action: new OR resume",);
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "new" | "n" => {
                return new_game();
            }
            "resume" | "e" => {
                return resume_game();
            }
            _ => {
                println!("Unknown action");
                continue;
            }
        };
    }
}

fn new_game() -> GameState {
    let player_count = parse_number_player(PLAYER_MAX_COUNT);
    return GameState::init(player_count);
}

fn resume_game() -> GameState {
    let path = Path::new(constant::SAVE_FOLDER).join(constant::SAVE_FILENAME);
    let save_content =
        fs::read_to_string(path).expect("Something went wrong reading the save file");
    return serde_json::from_str(&save_content).unwrap();
}
