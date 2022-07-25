use crate::constant::{PLAYER_MAX_COUNT, SAVE_FILENAME, SAVE_FOLDER};
use domain::application::Application;
use domain::game_state::GameState;
use domain::game_state_repository::GameStateRepository;
use infrastructure::game_state_repository_file_system::GameStateRepositoryFileSystem;
use infrastructure::parser::MainMenuAction;
use infrastructure::parser::{parse_party_selection, parse_player_names};

mod constant;
mod domain;
mod infrastructure;

fn main() {
    let main_menu_action = parse_party_selection();
    let game_state_repository = GameStateRepositoryFileSystem::new(SAVE_FOLDER, SAVE_FILENAME);

    let game_state = match main_menu_action {
        MainMenuAction::NewGame => {
            let player_names = parse_player_names(PLAYER_MAX_COUNT);
            GameState::new(player_names)
        }
        MainMenuAction::ResumeGame => game_state_repository.load(),
    };


    let mut app = Application::new(game_state, &game_state_repository);

    app.run();
}
