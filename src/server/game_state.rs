use crate::server::domino::Domino;
use crate::server::domino::DOMINOS;
use crate::server::player::Player;

// GameState handle core logic and low level manipulations.
// Source of truth
#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub index_current_player: usize, // index in players Vector of the current player
    pub dominos: Vec<Domino>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            index_current_player: 0,
            dominos: DOMINOS.to_vec(),
        }
    }
}
