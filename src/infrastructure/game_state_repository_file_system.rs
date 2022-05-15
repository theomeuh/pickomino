use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

use crate::domain::game_state_repository::GameStateRepository;
use crate::game_state::GameState;

pub struct GameStateRepositoryFileSystem<'a> {
    save_folder: &'a str,
    save_filename: &'a str,
}

impl GameStateRepositoryFileSystem<'_> {
    pub fn new<'a>(
        save_folder: &'a str,
        save_filename: &'a str,
    ) -> GameStateRepositoryFileSystem<'a> {
        return GameStateRepositoryFileSystem {
            save_filename: save_filename,
            save_folder: save_folder,
        };
    }
}

impl GameStateRepository for GameStateRepositoryFileSystem<'_> {
    fn save(&self, game_state: &GameState) {
        let serialized_game = serde_json::to_string(game_state).unwrap();

        match fs::create_dir(self.save_folder) {
            Ok(_) => {}
            Err(error) => match error.kind() {
                ErrorKind::AlreadyExists => {}
                _ => panic!("Cannot create save folder: {:?}", error),
            },
        };

        let path = Path::new(self.save_folder).join(self.save_filename);
        let mut file = File::create(path).expect("Cannot create save file");

        file.write_all(serialized_game.as_bytes())
            .expect("Cannot write save");
        println!("Party saved");
    }

    fn load(&self) -> GameState {
        let path = Path::new(self.save_folder).join(self.save_filename);
        let save_content =
            fs::read_to_string(path).expect("Something went wrong reading the save file");
        return serde_json::from_str(&save_content).unwrap();
    }
}
