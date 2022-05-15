use std::fmt;

use crate::domain::error::PickominoError;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DieLabel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Maggot = 6,
}

impl DieLabel {
    pub fn from(label: &str) -> Result<DieLabel, PickominoError> {
        match label {
            "one" | "One" => Ok(DieLabel::One),
            "two" | "Two" => Ok(DieLabel::Two),
            "three" | "Three" => Ok(DieLabel::Three),
            "four" | "Four" => Ok(DieLabel::Four),
            "five" | "Five" => Ok(DieLabel::Five),
            "maggot" | "Maggot" => Ok(DieLabel::Maggot),
            _ => Err(PickominoError::UnknownDiceLabel),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Die {
    pub label: DieLabel,
    pub value: u8,
    pub sort_value: u8,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.label)
    }
}

impl Die {
    pub fn from(label: &DieLabel) -> Die {
        match label {
            DieLabel::One => Die {
                label: DieLabel::One,
                value: 1,
                sort_value: 1,
            },
            DieLabel::Two => Die {
                label: DieLabel::Two,
                value: 2,
                sort_value: 2,
            },
            DieLabel::Three => Die {
                label: DieLabel::Three,
                value: 3,
                sort_value: 3,
            },
            DieLabel::Four => Die {
                label: DieLabel::Four,
                value: 4,
                sort_value: 4,
            },
            DieLabel::Five => Die {
                label: DieLabel::Five,
                value: 5,
                sort_value: 5,
            },
            DieLabel::Maggot => Die {
                label: DieLabel::Maggot,
                value: 5,
                sort_value: 6,
            },
        }
    }

    fn roll() -> Die {
        match rand::thread_rng().gen_range(1..=6) {
            1 => Die {
                label: DieLabel::One,
                value: 1,
                sort_value: 1,
            },
            2 => Die {
                label: DieLabel::Two,
                value: 2,
                sort_value: 2,
            },
            3 => Die {
                label: DieLabel::Three,
                value: 3,
                sort_value: 3,
            },
            4 => Die {
                label: DieLabel::Four,
                value: 4,
                sort_value: 4,
            },
            5 => Die {
                label: DieLabel::Five,
                value: 5,
                sort_value: 5,
            },
            6 => Die {
                label: DieLabel::Maggot,
                value: 5,
                sort_value: 6,
            },
            _ => unreachable!(),
        }
    }
}

pub struct PrintVecDie(pub Vec<Die>);
impl fmt::Display for PrintVecDie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sorted_dice = self.0.to_vec(); // copy vector
        sorted_dice.sort_unstable_by_key(|die| die.sort_value); // sort it
        for die in sorted_dice {
            write!(f, "{:?} ", die.label)?
        }
        Ok(())
    }
}
pub fn roll_dice(number_dice: usize) -> Vec<Die> {
    let mut dice = Vec::with_capacity(number_dice);
    for _ in 0..number_dice {
        dice.push(Die::roll())
    }
    dice
}
