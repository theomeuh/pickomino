use serde::{Deserialize, Serialize};

use crate::domain::dice::*;
use crate::domain::domino::*;

use crate::constant::DICE_COUNT;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub domino_stack: Vec<Domino>, // a player stack can contains at most the total number of domino
    pub dice_drawn: Vec<Die>,      // dice already drawn
}

impl PlayerState {
    pub fn init() -> PlayerState {
        PlayerState {
            domino_stack: Vec::with_capacity(DOMINO_COUNT),
            dice_drawn: Vec::with_capacity(DICE_COUNT),
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
        DICE_COUNT - self.dice_drawn.len()
    }
    pub fn has_maggot(&self) -> bool {
        for die in self.dice_drawn.iter() {
            if die.label == DieLabel::Maggot {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub state: PlayerState,
}
