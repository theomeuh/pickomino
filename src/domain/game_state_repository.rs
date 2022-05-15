use crate::game_state::GameState;

pub trait GameStateRepository {
    fn save(&self, game_state: &GameState);
    fn load(&self) -> GameState;
}
