use std::io;
use pickomino::GameState;

const PLAYER_MAX_COUNT: usize = 8;


fn main() {
    /////////////// The actual main function ///////////////
    let player_count = parse_number_player(PLAYER_MAX_COUNT);

    let mut game = GameState::init(player_count);
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
