use std::io;

use crate::constant::*;
use crate::infrastructure::main_menu::*;
use crate::infrastructure::shell_display_utility::*;
use crate::gamestate::GameState;

pub fn parse_number_player(max_player: usize) -> usize {
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

pub fn parse_party_selection() -> GameState {
    loop {
        print_seperator_shell();
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

pub fn parse_player_name() -> String {
    let mut player_name = String::with_capacity(MAX_SIZE_PLAYER_NAME);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name");

    String::from(player_name.trim())
}
