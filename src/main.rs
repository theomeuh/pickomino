use std::io;

pub mod dice {
    use rand::Rng;
    use std::fmt;

    pub const DICE_COUNT: usize = 8;

    #[derive(Debug)]
    enum DieValue {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Maggot = 6,
    }

    struct Die {
        label: DieValue,
        value: u8,
    }

    impl fmt::Display for Die {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.label)
        }
    }

    impl Die {
        fn roll() -> Die {
            match rand::thread_rng().gen_range(1..=6) {
                1 => Die {
                    label: DieValue::One,
                    value: 1,
                },
                2 => Die {
                    label: DieValue::Two,
                    value: 2,
                },
                3 => Die {
                    label: DieValue::Three,
                    value: 3,
                },
                4 => Die {
                    label: DieValue::Four,
                    value: 4,
                },
                5 => Die {
                    label: DieValue::Five,
                    value: 5,
                },
                6 => Die {
                    label: DieValue::Maggot,
                    value: 5,
                },
                _ => unreachable!(),
            }
        }
    }

    pub struct DiceRoll {
        dice: [Die; DICE_COUNT],
    }

    impl fmt::Display for DiceRoll {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                self.dice[0].label,
                self.dice[1].label,
                self.dice[2].label,
                self.dice[3].label,
                self.dice[4].label,
                self.dice[5].label,
                self.dice[6].label,
                self.dice[7].label
            )
        }
    }

    impl DiceRoll {
        pub fn roll() -> DiceRoll {
            DiceRoll {
                dice: [
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                    Die::roll(),
                ],
            }
        }
    }
}

pub mod engine {
    use std::fmt;
    use std::io;

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
        total: u8,
        remaining_dice: usize,
    }

    impl PlayerState {
        fn init() -> PlayerState {
            PlayerState {
                domino_stack: Vec::with_capacity(DOMINO_COUNT),
                total: 0,
                remaining_dice: crate::dice::DICE_COUNT,
            }
        }
    }

    #[derive(Debug)]
    struct Player {
        name: String,
        state: PlayerState,
        is_bot: bool,
    }

    pub struct GameState {
        players: Vec<Player>,
        current_player: u8, // index in players Vector of the current player
        dominos: [Domino; DOMINO_COUNT],
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

    impl GameState {
        pub fn init(player_count: usize) -> GameState {
            let mut players = Vec::with_capacity(player_count);
            for i in 0..player_count {
                let player_name = parse_player_name();
                players.push(Player {
                    name: player_name,
                    state: PlayerState::init(),
                    is_bot: false,
                });

                println!("{:?}", players[i]);
            }

            GameState {
                players: players,
                current_player: 0,
                dominos: DOMINOS,
            }
        }
        pub fn run(&self) {
            println!("Game start");
            let finished = false;
            let mut round_number = 0;

            while !finished && round_number <= MAX_ROUND {
                round_number += 1;
                for player in self.players.iter() {
                    println!("{} state, total {}", player.name, player.state.total)
                }
            }
            println!("Game end");
        }
    }
}

// pub mod cli {
//     pub fn parse_action_input(action: &str) -> Option<PlayerAction> {
//         match action {
//             "1" => Some(PlayerAction::Draw),
//             "2" => Some(PlayerAction::Roll),
//             "3" => Some(PlayerAction::Surrend),
//             _ => {
//                 println!("Invalid action number");
//                 None
//             }
//         }
//     }
// }

fn main() {
    /////////////// The actual main function ///////////////
    const PLAYER_MAX_COUNT: usize = 8;
    let player_count = parse_number_player(PLAYER_MAX_COUNT);

    let game = engine::GameState::init(player_count);
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
