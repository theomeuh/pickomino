use crate::dice;
use crate::domino;
use std::io;

pub const MAX_SIZE_PLAYER_NAME: usize = 50;

#[derive(Debug)]
pub struct PlayerState {
    pub domino_stack: Vec<domino::Domino>, // a player stack can contains at most the total number of domino
    pub dice_drawn: Vec<dice::Die>,        // dice already drawn
}

impl PlayerState {
    pub fn init() -> PlayerState {
        PlayerState {
            domino_stack: Vec::with_capacity(domino::DOMINO_COUNT),
            dice_drawn: Vec::with_capacity(crate::DICE_COUNT),
        }
    }
    pub fn dice_total(&self) -> u8 {
        let mut total: u8 = 0;
        for die in self.dice_drawn.iter() {
            total += die.value;
        }
        total
    }
    pub fn rollable_dice_count(&self) -> usize {
        crate::DICE_COUNT - self.dice_drawn.len()
    }
    pub fn has_maggot(&self) -> bool {
        for die in self.dice_drawn.iter() {
            if die.label == dice::DieLabel::Maggot {
                return true;
            }
        }
        return false;
    }
    pub fn domino_total(&self) -> u8 {
        let mut total: u8 = 0;
        for domino in self.domino_stack.iter() {
            total += domino.value;
        }
        total
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub state: PlayerState,
}

pub fn parse_player_name() -> String {
    let mut player_name = String::with_capacity(MAX_SIZE_PLAYER_NAME);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name");

    String::from(player_name.trim())
}
