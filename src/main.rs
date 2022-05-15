use crate::gamestate::GameState;
use infrastructure::parser::parse_party_selection;

mod constant;
mod domain;
mod gamestate;
mod infrastructure;

fn main() {
    let mut game: GameState = parse_party_selection();
    game.run();
}
