use rand::Rng;
use std::fmt;
use std::io;
use std::result::Result;

const MAX_SIZE_PLAYER_NAME: usize = 50;

pub const DICE_COUNT: usize = 8;

const DOMINO_COUNT: usize = 16;
const DOMINOS: [Domino; DOMINO_COUNT] = [
    Domino {
        label: 21,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 22,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 23,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 24,
        value: 1,
        pickable: true,
    },
    Domino {
        label: 25,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 26,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 27,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 28,
        value: 2,
        pickable: true,
    },
    Domino {
        label: 29,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 30,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 31,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 32,
        value: 3,
        pickable: true,
    },
    Domino {
        label: 33,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 34,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 35,
        value: 4,
        pickable: true,
    },
    Domino {
        label: 36,
        value: 4,
        pickable: true,
    },
];

#[derive(Debug)]
pub enum PickominoError {
    NoDiceToRoll,
    UnknownDiceLabel,
    DominoTooBig,
    CancelAction,
}

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

#[derive(Clone, Debug)]
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

    pub fn roll() -> Die {
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

struct PrintVecDie(Vec<Die>);
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

struct Domino {
    label: u8,
    value: u8,
    pickable: bool,
}

impl fmt::Debug for Domino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [pickable={}]", self.label, self.pickable)
    }
}

impl Domino {
    pub fn from(label: u8) -> Domino {
        match label {
            // find shorter way to do this
            21 => Domino {
                label: 21,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            22 => Domino {
                label: 22,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            23 => Domino {
                label: 23,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            24 => Domino {
                label: 24,
                value: 1,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            25 => Domino {
                label: 25,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            26 => Domino {
                label: 26,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            27 => Domino {
                label: 27,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            28 => Domino {
                label: 28,
                value: 2,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            29 => Domino {
                label: 29,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            30 => Domino {
                label: 30,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            31 => Domino {
                label: 31,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            32 => Domino {
                label: 32,
                value: 3,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            33 => Domino {
                label: 33,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            34 => Domino {
                label: 34,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            35 => Domino {
                label: 35,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            36 => Domino {
                label: 36,
                value: 4,
                pickable: true, // not relevant for a "Player Domino". Need refacto Domino handling
            },
            _ => panic!("Wrong label"),
        }
    }
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
    fn has_maggot(&self) -> bool {
        for die in self.dice_drawn.iter() {
            if die.label == DieLabel::Maggot {
                return true;
            }
        }
        return false;
    }
    fn domino_total(&self) -> u8 {
        let mut total: u8 = 0;
        for domino in self.domino_stack.iter() {
            total += domino.value;
        }
        total
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    state: PlayerState,
}

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    index_current_player: usize, // index in players Vector of the current player
    dominos: [Domino; DOMINO_COUNT],
    finished: bool,
}

pub fn parse_player_name() -> String {
    let mut player_name = String::with_capacity(MAX_SIZE_PLAYER_NAME);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name");

    String::from(player_name.trim())
}

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
            dominos: DOMINOS,
            finished: false,
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.index_current_player]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.index_current_player]
    }

    pub fn select_next_player(&mut self) {
        self.index_current_player = (self.index_current_player + 1) % self.players.len()
    }

    pub fn roll_dice(&self) -> Vec<Die> {
        let rollable_dice_count = self.current_player().state.rollable_dice_count();
        _roll_dice(rollable_dice_count)
    }

    pub fn add_dice_to_current_player(&mut self, count: usize, label: &DieLabel) {
        for _ in 0..count {
            self.current_player_mut()
                .state
                .dice_drawn
                .push(Die::from(label))
        }
    }

    pub fn println_pickable_dominos(&self) {
        for domino in self.dominos.iter() {
            if domino.pickable {
                print!("{:?} ", domino.label)
            }
        }
        println!()
    }

    pub fn count_pickable_dice_by_label(&self, dice: &Vec<Die>, label: &DieLabel) -> usize {
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

    pub fn draw_dice(&mut self) -> Result<(), PickominoError> {
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
    pub fn pick_domino(&mut self) -> Result<(), PickominoError> {
        println!("'pick' selected");
        println!(
            "You can pick a domino up to {:?}. Minimum is 21",
            self.current_player().state.dice_total()
        );

        let mut domino_value: u8;
        let mut domino_index: usize;
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

            // validity check
            if !self.dominos.get(domino_index).unwrap().pickable {
                println!("This domino is not on the board. Pick another");
                continue;
            }
            break;
        }

        // make the change
        self.current_player_mut()
            .state
            .domino_stack
            .push(Domino::from(domino_value));
        self.dominos.get_mut(domino_index).unwrap().pickable = false;

        Ok(())
    }
    pub fn show_current_player_state(&self) {
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
    pub fn play_current_player(&mut self) {
        clear_shell();
        loop {
            print_seperator_shell();
            println!(
                "{:} please, select an action: draw OR pick OR show",
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
    pub fn is_finished(&self) -> bool {
        for domino in self.dominos.iter() {
            if domino.pickable == true {
                return false;
            }
        }
        return true;
    }
    pub fn run(&mut self) {
        println!("Game start");

        while self.is_finished() {
            self.play_current_player();
            self.select_next_player();
        }
        println!("Game end");
    }
}

/// print special shell character to clear a shell
pub fn clear_shell() {
    print!("\x1B[2J");
}

/// use to separate group of message
pub fn print_seperator_shell() {
    println!("--------------------------------");
}
