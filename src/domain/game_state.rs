use serde::{Deserialize, Serialize};

use crate::domain::domino::Domino;
use crate::domain::domino::DOMINOS;
use crate::domain::player::{Player, PlayerState};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub index_current_player: usize, // index in players Vector of the current player
    pub dominos: Vec<Domino>,
}

impl GameState {
    pub fn new(player_names: Vec<String>) -> GameState {
        let mut players = Vec::with_capacity(player_names.len());
        for player_name in player_names.into_iter() {
            players.push(Player {
                name: player_name,
                state: PlayerState::init(),
            });
        }

        GameState {
            players: players,
            index_current_player: 0,
            dominos: DOMINOS.to_vec(),
        }
    }
}

pub trait GameStateRepository {
    fn save(&self, game_state: &GameState);
    fn load(&self) -> GameState;
}
