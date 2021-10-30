use std::io;

use rand::Rng;
use std::fmt;

pub const DICE_COUNT: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub enum DieLabel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Maggot = 6,
}

impl DieLabel {
    pub fn from(label: &str) -> DieLabel {
        match label {
            "one" | "One" => DieLabel::One,
            "two" | "Two" => DieLabel::Two,
            "three" | "Three" => DieLabel::Three,
            "four" | "Four" => DieLabel::Four,
            "five" | "Five" => DieLabel::Five,
            "maggot" | "Maggot" => DieLabel::Maggot,
            _ => panic!("Wrong label"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Die {
    pub label: DieLabel,
    pub value: u8,
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
            },
            DieLabel::Two => Die {
                label: DieLabel::Two,
                value: 2,
            },
            DieLabel::Three => Die {
                label: DieLabel::Three,
                value: 3,
            },
            DieLabel::Four => Die {
                label: DieLabel::Four,
                value: 4,
            },
            DieLabel::Five => Die {
                label: DieLabel::Five,
                value: 5,
            },
            DieLabel::Maggot => Die {
                label: DieLabel::Maggot,
                value: 5,
            },
        }
    }

    pub fn roll() -> Die {
        match rand::thread_rng().gen_range(1..=6) {
            1 => Die {
                label: DieLabel::One,
                value: 1,
            },
            2 => Die {
                label: DieLabel::Two,
                value: 2,
            },
            3 => Die {
                label: DieLabel::Three,
                value: 3,
            },
            4 => Die {
                label: DieLabel::Four,
                value: 4,
            },
            5 => Die {
                label: DieLabel::Five,
                value: 5,
            },
            6 => Die {
                label: DieLabel::Maggot,
                value: 5,
            },
            _ => unreachable!(),
        }
    }
}

struct PrintVecDie(Vec<Die>);
impl fmt::Display for PrintVecDie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sorted_dice = self.0.to_vec(); // copy vector
        sorted_dice.sort_unstable_by_key(|die| die.value); // sort it
        for die in sorted_dice {
            write!(f, "{:?} ", die.label)?
        }
        Ok(())
    }
}

const DOMINO_COUNT: usize = 16;
const MAX_ROUND: usize = 1;

struct Domino {
    label: u8,
    value: u8,
    active: bool,
}

impl fmt::Debug for Domino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Debug)]
pub enum PlayerAction {
    Roll,
    Draw,
    Surrend,
}

#[derive(Debug)]
struct PlayerState {
    domino_stack: Vec<Domino>, // a player stack can contains at most the total number of domino
    dice_drawn: Vec<Die>,      // dice already drawn
}

impl PlayerState {
    fn init() -> PlayerState {
        PlayerState {
            domino_stack: Vec::with_capacity(DOMINO_COUNT),
            dice_drawn: Vec::with_capacity(DICE_COUNT),
        }
    }
    fn dice_total(&self) -> u8 {
        let mut total: u8 = 0;
        for die in self.dice_drawn.iter() {
            total += die.value;
        }
        total
    }
    fn rollable_dice_count(&self) -> usize {
        DICE_COUNT - self.dice_drawn.len()
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    state: PlayerState,
}

pub struct GameState {
    players: Vec<Player>,
    index_current_player: usize, // index in players Vector of the current player
    dominos: [Domino; DOMINO_COUNT],
    finished: bool,
    round_number: usize,
}

const DOMINOS: [Domino; 16] = [
    Domino {
        label: 21,
        value: 1,
        active: true,
    },
    Domino {
        label: 22,
        value: 1,
        active: true,
    },
    Domino {
        label: 23,
        value: 1,
        active: true,
    },
    Domino {
        label: 24,
        value: 1,
        active: true,
    },
    Domino {
        label: 25,
        value: 2,
        active: true,
    },
    Domino {
        label: 26,
        value: 2,
        active: true,
    },
    Domino {
        label: 27,
        value: 2,
        active: true,
    },
    Domino {
        label: 28,
        value: 2,
        active: true,
    },
    Domino {
        label: 29,
        value: 3,
        active: true,
    },
    Domino {
        label: 30,
        value: 3,
        active: true,
    },
    Domino {
        label: 31,
        value: 3,
        active: true,
    },
    Domino {
        label: 32,
        value: 3,
        active: true,
    },
    Domino {
        label: 33,
        value: 4,
        active: true,
    },
    Domino {
        label: 34,
        value: 4,
        active: true,
    },
    Domino {
        label: 35,
        value: 4,
        active: true,
    },
    Domino {
        label: 36,
        value: 4,
        active: true,
    },
];

const MAX_SIZE_PLAYER_NAME: usize = 50;

pub fn parse_player_name() -> String {
    println!("Enter player name");

    let mut player_name = String::with_capacity(MAX_SIZE_PLAYER_NAME);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name");

    String::from(player_name.trim())
}

pub fn parse_action_input(state: &mut GameState) {}

pub fn _roll_dice(number_dice: usize) -> Vec<Die> {
    let mut dice = Vec::with_capacity(number_dice);
    for _ in 0..number_dice {
        dice.push(Die::roll())
    }
    dice
}

impl GameState {
    pub fn init(player_count: usize) -> GameState {
        let mut players = Vec::with_capacity(player_count);
        for i in 0..player_count {
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
            dominos: DOMINOS,
            finished: false,
            round_number: 0,
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.finished && self.round_number <= MAX_ROUND
    }

    pub fn select_next_player(&mut self) {
        self.index_current_player = (self.index_current_player + 1) % self.players.len()
    }

    pub fn roll_dice(&self) -> Vec<Die> {
        let rollable_dice_count = self.players[self.index_current_player]
            .state
            .rollable_dice_count();
        _roll_dice(rollable_dice_count)
    }

    pub fn add_dice_to_current_player(&mut self, count: usize, label: &DieLabel) {
        for _ in 0..count {
            self.players[self.index_current_player]
                .state
                .dice_drawn
                .push(Die::from(label))
        }
    }

    pub fn count_pickable_dice(&self, dice: &Vec<Die>, label: &DieLabel) -> usize {
        let count = dice.iter().filter(|die| &die.label == label).count();
        if count == 0 {
            panic!("dice not proposed");
        }
        if self.players[self.index_current_player]
            .state
            .dice_drawn
            .iter()
            .filter(|die| &die.label == label)
            .count()
            != 0
        {
            panic!("dice already drawn by player");
        }
        count
    }

    pub fn draw(&mut self, dice: &Vec<Die>) {
        // read input
        println!("Select a die label");
        let mut label = String::new();
        io::stdin()
            .read_line(&mut label)
            .expect("Failed to read line");
        let label = DieLabel::from(label.trim());

        // make stuff
        let count = self.count_pickable_dice(dice, &label);
        self.add_dice_to_current_player(count, &label);

        // print current state
        println!("{:?}", self.players[self.index_current_player])
    }
    pub fn play_current_player(&mut self) {
        let rolled_dice = self.roll_dice();
        println!("You rolled {}", PrintVecDie(rolled_dice.clone()));

        println!("Select an action: draw");
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim() {
            "draw" => {
                println!("draw");
                self.draw(&rolled_dice)
            }
            _ => panic!("unknown action"),
        }
    }
    pub fn run(&mut self) {
        println!("Game start");

        while self.is_finished() {
            self.round_number += 1;
            self.select_next_player();
            self.play_current_player();

            self.finished = true; // to test
        }
        println!("Game end");
    }
}

fn main() {
    /////////////// The actual main function ///////////////
    const PLAYER_MAX_COUNT: usize = 8;
    let player_count = parse_number_player(PLAYER_MAX_COUNT);

    let mut game = GameState::init(player_count);
    game.run();
}

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
