use crate::server::game_state::GameState;

use strum::IntoEnumIterator;

use super::actions::Action;
// Initialize the game state, handle requests and manipulate the game state
// Can also load a save
pub struct Server {
    game_state: GameState,
}

impl Server {
    pub fn new() -> Server {
        let game_state = GameState::new();

        Server { game_state }
    }
    pub fn list_actions(&self) -> Vec<Action> {
        Action::iter().collect()
    }
}
