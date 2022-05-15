use serde::{Deserialize, Serialize};

use crate::domain::domino::{Domino, DOMINOS};
use crate::domain::player::{Player, PlayerState};
use crate::infrastructure::parser::parse_player_name;

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
}
