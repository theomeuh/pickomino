use std::io;

use crate::constant::*;
use crate::infrastructure::shell_display_utility::*;

pub fn parse_player_names(max_player: usize) -> Vec<String> {
    let player_count = parse_player_count(max_player);

    let mut players = Vec::with_capacity(max_player);

    for i in 0..player_count {
        println!("Enter name of player {:?}", (i + 1));
        let player_name = parse_player_name();
        players.push(player_name)
    }
    players
}

pub fn parse_player_count(max_player: usize) -> usize {
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

pub fn parse_party_selection() -> MainMenuAction {
    loop {
        print_seperator_shell();
        println!("Please select an action: new OR resume",);
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "new" | "n" => {
                return MainMenuAction::NewGame;
            }
            "resume" | "r" => {
                return MainMenuAction::ResumeGame;
            }
            _ => {
                println!("Unknown action");
                continue;
            }
        };
    }
}

#[derive(Debug)]
pub enum MainMenuAction {
    NewGame = 1,
    ResumeGame = 2,
}

pub fn parse_player_name() -> String {
    let mut player_name = String::with_capacity(MAX_SIZE_PLAYER_NAME);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name");

    String::from(player_name.trim())
}
