use crate::constant::{SAVE_FILENAME, SAVE_FOLDER};
use crate::game_state::GameState;
use domain::application::Application;
use infrastructure::game_state_repository_file_system::GameStateRepositoryFileSystem;
use infrastructure::parser::parse_party_selection;

mod constant;
mod domain;
mod game_state;
mod infrastructure;

fn main() {
    let game_state: GameState = parse_party_selection();
    let game_state_repository = GameStateRepositoryFileSystem::new(SAVE_FOLDER, SAVE_FILENAME);
    let mut app = Application::new(game_state, &game_state_repository);

    app.run();
}
