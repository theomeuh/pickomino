use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::result::Result;

use serde::{Deserialize, Serialize};

use crate::domain::dice::{roll_dice, Die, DieLabel, PrintVecDie};
use crate::domain::domino::{Domino, DOMINOS};
use crate::domain::error::PickominoError;
use crate::domain::player::{Player, PlayerState};
use crate::infrastructure::parser::parse_player_name;
use crate::infrastructure::shell_display_utility::*;
use crate::constant::{SAVE_FILENAME, SAVE_FOLDER};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub index_current_player: usize, // index in players Vector of the current player
    pub dominos: Vec<Domino>,
}

impl GameState {
    pub fn init(player_count: usize) -> GameState {
        let mut players = Vec::with_capacity(player_count);
        for i in 0..player_count {
            println!("Enter name of player {:?}", (i + 1));
            let player_name = parse_player_name();
            players.push(Player {
                name: player_name,
                state: PlayerState::init(),
            });

            println!("{:?}", players[i]);
        }

        GameState {
            players: players,
            index_current_player: 0,
            dominos: DOMINOS.to_vec(),
        }
    }

    fn current_player(&self) -> &Player {
        &self.players[self.index_current_player]
    }

    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.index_current_player]
    }

    fn select_next_player(&mut self) {
        self.index_current_player = (self.index_current_player + 1) % self.players.len()
    }

    fn roll_dice(&self) -> Vec<Die> {
        let rollable_dice_count = self.current_player().state.rollable_dice_count();
        roll_dice(rollable_dice_count)
    }

    fn add_dice_to_current_player(&mut self, count: usize, label: &DieLabel) {
        for _ in 0..count {
            self.current_player_mut()
                .state
                .dice_drawn
                .push(Die::from(label))
        }
    }

    fn println_pickable_dominos(&self) {
        for domino in self.dominos.iter() {
            print!("{:?} ", domino.label)
        }
        println!()
    }

    fn count_pickable_dice_by_label(&self, dice: &Vec<Die>, label: &DieLabel) -> usize {
        // count how many die have the label in the last draw
        let count = dice.iter().filter(|die| &die.label == label).count();
        if count == 0 {
            return 0;
        }
        // check if the dice selected have already been drawn by the current player
        if self
            .current_player()
            .state
            .dice_drawn
            .iter()
            .filter(|die| &die.label == label)
            .count()
            != 0
        {
            return 0;
        }
        count
    }

    fn draw_dice(&mut self) -> Result<(), PickominoError> {
        println!("'draw' selected");
        print_seperator_shell();
        if self.current_player().state.rollable_dice_count() == 0 {
            return Err(PickominoError::NoDiceToRoll);
        }
        let rolled_dice = self.roll_dice();
        loop {
            // read input
            println!("You rolled {}", PrintVecDie(rolled_dice.clone()));
            println!("Select a die label");
            let mut label = String::new();
            io::stdin()
                .read_line(&mut label)
                .expect("Failed to read line");
            let label = match DieLabel::from(label.trim()) {
                Ok(label) => label,
                Err(_) => {
                    print_seperator_shell();
                    println!("Wrong Die Label");
                    continue;
                }
            };

            // make stuff
            let count = self.count_pickable_dice_by_label(&rolled_dice, &label);
            if count == 0 {
                // loop if no die can be drawn
                println!("Dice '{:?}' cannot be drawn", label);
                continue;
            } else {
                // break if there is dice drawn
                self.add_dice_to_current_player(count, &label);
                break;
            }
        }
        Ok(())
    }
    fn pick_domino(&mut self) -> Result<(), PickominoError> {
        println!("'pick' selected");
        println!(
            "You can pick a domino up to {:?}. Minimum is 21",
            self.current_player().state.dice_total()
        );

        let mut domino_value: u8;
        let domino_index: usize;
        loop {
            // read input
            println!("Select a domino label or type 'cancel'");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "cancel" | "c" => return Err(PickominoError::CancelAction),
                _ => {}
            }
            domino_value = input.trim().parse::<u8>().unwrap();

            if domino_value > self.current_player().state.dice_total() {
                println!("You cannot pick this domino. Pick a lower one");
                continue;
            }

            // find the domino on the board game
            domino_index = self
                .dominos
                .iter()
                .position(|domino| domino.label == domino_value)
                .unwrap();
            break;
        }

        // make the change
        let picked_domino = self.dominos.swap_remove(domino_index);
        self.current_player_mut()
            .state
            .domino_stack
            .push(picked_domino);
        Ok(())
    }
    fn show_current_player_state(&self) {
        println!();
        println!(
            "Remaining dice to draw {:?}",
            self.current_player().state.rollable_dice_count()
        );
        println!(
            "Total of collected dice: {:?}",
            self.current_player().state.dice_total()
        );
        println!("Has Maggot: {:?}", self.current_player().state.has_maggot());
        println!(
            "Total of collected domino: {:?}",
            self.current_player().state.domino_total()
        );
        println!("Available domino on the board game");
        self.println_pickable_dominos();
    }
    fn play_current_player(&mut self) {
        clear_shell();
        loop {
            print_seperator_shell();
            println!(
                "{:} please, select an action: draw OR pick OR show OR save",
                self.current_player().name
            );
            let mut action = String::new();

            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");

            let action_result = match action.trim() {
                "draw" | "d" => self.draw_dice(),
                "pick" | "p" => self.pick_domino(),
                "show" | "s" => {
                    self.show_current_player_state();
                    continue;
                }
                "save" => {
                    self.save_party();
                    continue;
                }
                _ => {
                    println!("Unknown action");
                    continue;
                }
            };
            match action_result {
                // re-ask action if previous action went bad
                Err(message) => {
                    println!("Error: {:?}", message);
                    continue;
                }
                // leave the loop if the action went well
                _ => break,
            }
        }
    }
    fn save_party(&self) {
        let serialized_game = serde_json::to_string(self).unwrap();

        match fs::create_dir(SAVE_FOLDER) {
            Ok(_) => {}
            Err(error) => match error.kind() {
                ErrorKind::AlreadyExists => {}
                _ => panic!("Cannot create save folder: {:?}", error),
            },
        };

        let path = Path::new(SAVE_FOLDER).join(SAVE_FILENAME);
        let mut file = File::create(path).expect("Cannot create save file");

        file.write_all(serialized_game.as_bytes())
            .expect("Cannot write save");
        println!("Party saved");
    }
    fn is_finished(&self) -> bool {
        return self.dominos.is_empty();
    }
    pub fn run(&mut self) {
        println!("Game start");

        while !self.is_finished() {
            self.play_current_player();
            self.select_next_player();
        }
        println!("Game end");
    }
}
